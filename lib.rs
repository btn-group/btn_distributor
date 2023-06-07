#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod btn_distributor {
    use ink::{
        env::{
            call::{build_call, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        storage::Mapping,
        LangError,
    };

    use openbrush::{
        contracts::ownable::*, contracts::traits::psp22::PSP22Error, modifiers, traits::Storage,
    };

    // === ENUMS ===
    // https://github.com/Brushfam/openbrush-contracts/blob/73783af2f887eeb1fb3ebb4a012d5f3ff4598b56/docs/docs/smart-contracts/example/errors.md?plain=1#L16
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum BtnDistributorError {
        OrderAlreadyProcessed,
        OwnableError(OwnableError),
        PSP22Error(PSP22Error),
    }
    impl From<PSP22Error> for BtnDistributorError {
        fn from(error: PSP22Error) -> Self {
            BtnDistributorError::PSP22Error(error)
        }
    }
    impl From<OwnableError> for BtnDistributorError {
        fn from(error: OwnableError) -> Self {
            BtnDistributorError::OwnableError(error)
        }
    }

    // === STRUCTS ===
    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Config {
        admin: AccountId,
        btn: SmartContract,
    }

    #[derive(Clone, Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct SmartContract {
        address: AccountId,
        code_hash: Hash,
    }

    #[ink(storage)]
    #[derive(Storage)]
    pub struct BtnDistributor {
        #[storage_field]
        ownable: ownable::Data,
        btn: SmartContract,
        order_details: Mapping<String, Balance>,
    }

    impl BtnDistributor {
        #[ink(constructor)]
        pub fn new(btn: SmartContract) -> Self {
            let mut instance = Self {
                ownable: Default::default(),
                btn,
                order_details: Mapping::default(),
            };
            instance._init_with_owner(Self::env().caller());
            instance
        }

        // === QUERY ===
        #[ink(message)]
        pub fn config(&self) -> Config {
            Config {
                admin: self.ownable.owner(),
                btn: self.btn.clone(),
            }
        }

        // === HANDLE ===
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
        ) -> Result<(), BtnDistributorError> {
            let call_result: Result<Result<(), PSP22Error>, ink::LangError> = build_call::<
                DefaultEnvironment,
            >()
            .call(self.btn.address)
            .gas_limit(0)
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!("decrease_allowance")))
                    .push_arg(spender)
                    .push_arg(delta_value),
            )
            .returns::<Result<Result<(), PSP22Error>, LangError>>()
            .invoke();
            match call_result {
                // An error emitted by the smart contracting language.
                // For more details see ink::LangError.
                Err(lang_error) => {
                    panic!("Unexpected ink::LangError: {:?}", lang_error)
                }
                // `Result<(), PSP22Error>` is the return type of
                // the method we're calling.
                Ok(Err(contract_call_error)) => Err(BtnDistributorError::from(contract_call_error)),
                Ok(Ok(())) => Ok(()),
            }
        }

        // https://use.ink/basics/cross-contract-calling#createbuilder
        // https://github.com/Brushfam/openbrush-contracts/blob/73783af2f887eeb1fb3ebb4a012d5f3ff4598b56/docs/docs/smart-contracts/example/errors.md?plain=1#L16
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: Balance,
            order_id: String,
        ) -> Result<(), BtnDistributorError> {
            if self.order_details.get(order_id.clone()).is_some() {
                return Err(BtnDistributorError::OrderAlreadyProcessed);
            } else {
                self.order_details.insert(order_id, &delta_value);
            }

            let call_result: Result<Result<(), PSP22Error>, ink::LangError> = build_call::<
                DefaultEnvironment,
            >()
            .call(self.btn.address)
            .gas_limit(0)
            .exec_input(
                ExecutionInput::new(Selector::new(ink::selector_bytes!("increase_allowance")))
                    .push_arg(spender)
                    .push_arg(delta_value),
            )
            .returns::<Result<Result<(), PSP22Error>, LangError>>()
            .invoke();
            match call_result {
                // An error emitted by the smart contracting language.
                // For more details see ink::LangError.
                Err(lang_error) => {
                    panic!("Unexpected ink::LangError: {:?}", lang_error)
                }
                // `Result<(), PSP22Error>` is the return type of
                // the method we're calling.
                Ok(Err(contract_call_error)) => Err(BtnDistributorError::from(contract_call_error)),
                Ok(Ok(())) => Ok(()),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::DefaultAccounts;
        use openbrush::test_utils;

        // === HELPER FUNCTIONS ===
        fn init() -> (DefaultAccounts<DefaultEnvironment>, BtnDistributor) {
            let accounts = test_utils::accounts();
            test_utils::change_caller(accounts.bob);
            let btn_distributor = BtnDistributor::new(mock_btn());
            (accounts, btn_distributor)
        }

        fn mock_btn() -> SmartContract {
            SmartContract {
                address: AccountId::try_from(*b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").unwrap(),
                code_hash: Hash::try_from(*b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxy").unwrap(),
            }
        }

        // === TESTS ===
        #[ink::test]
        fn test_new() {
            let (accounts, btn_distributor) = init();
            // * it sets owner as caller
            assert_eq!(btn_distributor.ownable.owner(), accounts.bob);
            // * it sets btn
            assert_eq!(btn_distributor.btn.address, mock_btn().address);
            assert_eq!(btn_distributor.btn.code_hash, mock_btn().code_hash);
        }

        #[ink::test]
        fn test_decrease_allowance() {
            let (accounts, mut btn_distributor) = init();
            // when called by a non-admin
            test_utils::change_caller(accounts.alice);
            // * it raises an error
            let result = btn_distributor.decrease_allowance(accounts.alice, 1_000_000);
            assert_eq!(
                result,
                Err(BtnDistributorError::OwnableError(
                    OwnableError::CallerIsNotOwner
                ))
            );
        }

        #[ink::test]
        fn test_increase_allowance() {
            let (accounts, mut btn_distributor) = init();
            // when called by a non-admin
            test_utils::change_caller(accounts.alice);
            // * it raises an error
            let mut result =
                btn_distributor.increase_allowance(accounts.alice, 1_000_000, "xxx".to_string());
            assert_eq!(
                result,
                Err(BtnDistributorError::OwnableError(
                    OwnableError::CallerIsNotOwner
                ))
            );
            // when called by an admin
            test_utils::change_caller(accounts.bob);
            // = when order_id exists
            btn_distributor
                .order_details
                .insert("xxx".to_string(), &1_000_000);
            // = * it raises an error
            result =
                btn_distributor.increase_allowance(accounts.alice, 1_000_000, "xxx".to_string());
            assert_eq!(result, Err(BtnDistributorError::OrderAlreadyProcessed));
            // = when order_id does not exist
            // = * it sets the order_id (This needs to be checked in staging)
        }
    }
}

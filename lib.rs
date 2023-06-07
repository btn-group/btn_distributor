#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod btn_distributor {
    use ink::{
        env::{
            call::{build_call, ExecutionInput, Selector},
            DefaultEnvironment,
        },
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
        PSP22Error(PSP22Error),
        OwnableError(OwnableError),
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
    }

    impl BtnDistributor {
        #[ink(constructor)]
        pub fn new(btn: SmartContract) -> Self {
            let mut instance = Self {
                ownable: Default::default(),
                btn,
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
        // https://use.ink/basics/cross-contract-calling#createbuilder
        // https://github.com/Brushfam/openbrush-contracts/blob/73783af2f887eeb1fb3ebb4a012d5f3ff4598b56/docs/docs/smart-contracts/example/errors.md?plain=1#L16
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn increase_allowance(
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
}

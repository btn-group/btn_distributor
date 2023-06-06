#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod btn_distributor {
    use openbrush::{contracts::ownable::*, traits::Storage};

    // === STRUCTS ===
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Config {
        admin: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct BtnDistributor {
        #[storage_field]
        ownable: ownable::Data,
    }

    impl BtnDistributor {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance
        }

        #[ink(message)]
        pub fn config(&self) -> Config {
            Config {
                admin: self.ownable.owner(),
            }
        }
    }
}

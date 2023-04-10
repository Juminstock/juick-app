#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod ink_bank {
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        UnsuffitientBalance,
        TransferFailed,
        OnlyOwner,
    }
    use ink::storage::Mapping;
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(storage)]
    pub struct Bank {
        owner: AccountId,
        balances: Mapping<AccountId, Balance>,
    }

    impl Bank {
        #[ink(constructor)]
        pub fn new() -> Self {
            let balances = Mapping::default();
            let caller = Self::env().caller();

            Self {
                owner: caller,
                balances,
            }
        }

        #[ink(message)]
        pub fn add_balance(&mut self) {
            let caller = Self::env().caller();
            let value = Self::env().transferred_value();
            let current_balance = self.balances.get(caller).unwrap_or_default();
            let new_balance = value + current_balance;

            self.balances.insert(caller, &new_balance);
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<()> {
            let caller = Self::env().caller();
            let current_balance = self.balances.get(caller).unwrap_or_default();

            if current_balance < amount {
                return Err(Error::UnsuffitientBalance);
            }

            if self.env().transfer(caller, amount).is_err() {
                return Err(Error::TransferFailed);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn stole_all(&mut self) -> Result<()> {
            let caller = Self::env().caller();

            if caller != self.owner {
                return Err(Error::OnlyOwner);
            }

            if self
                .env()
                .transfer(self.owner, self.env().balance())
                .is_err()
            {
                return Err(Error::TransferFailed);
            }

            Ok(())
        }
    }
}
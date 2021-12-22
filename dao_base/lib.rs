#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_base::{
    DaoBase,
};

#[ink::contract]
mod dao_base {

    use alloc::string::String;
    #[ink(storage)]
    pub struct DaoBase {
        owner: AccountId,
        name: String,
        synopsis:String,
        symbol:String,
    }

    impl DaoBase {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                name:String::default(),
                synopsis:String::default(),
                symbol:String::default(),
            }
        }
        #[ink(message)]
        pub fn init_base(&mut self, name: String, synopsis: String, symbol: String) {
            self.set_name(name);
            self.set_synopsis(synopsis);
            self.set_symbol(symbol);
        }
        #[ink(message)]
        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }
        #[ink(message)]
        pub fn set_synopsis(&mut self, synopsis: String) {
            self.synopsis = synopsis;
        }
        #[ink(message)]
        pub fn set_symbol(&mut self, symbol: String) {
            self.symbol = symbol;
        }



    }
}

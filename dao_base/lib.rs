#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::dao_base::{
    DaoBase,
};

#[ink::contract]
mod dao_base {

    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std", 
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DaoInfo {
        owner: AccountId,
        name: String,
        synopsis: String,
        symbol: String,
    }
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
        pub fn get_name(&self) -> String{
            self.name.clone()
        }

        #[ink(message)]
        pub fn set_synopsis(&mut self, synopsis: String) {
            self.synopsis = synopsis;
        }

        #[ink(message)]
        pub fn get_synopsis(&self) -> String{
            self.synopsis.clone()
        }

        #[ink(message)]
        pub fn set_symbol(&mut self, symbol: String) {
            self.symbol = symbol;
        }

        #[ink(message)]
        pub fn get_symbol(&self) -> String{
            self.symbol.clone()
        }

        #[ink(message)]
        pub fn get_baseInfo(&self) ->DaoInfo{
            DaoInfo{
                owner: self.owner,
                name: self.name.clone(),
                synopsis: self.synopsis.clone(),
                symbol: self.symbol.clone(),
            }
        }



    }
}

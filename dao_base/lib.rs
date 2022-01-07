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

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
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
        pub fn set_owner(&mut self, creator: AccountId) {

            let owner = self.env().caller();

            if self.owner == AccountId::default() || owner == self.owner {
                self.owner = owner;
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
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

    #[cfg(test)]

    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_name() {
            let mut base = DaoBase::default();
            base.set_name("DaoBase".to_string());
            let dbg_msg = format!("name is {}", base.get_name());
            ink_env::debug_println!("{}", &dbg_msg );
            assert_eq!(base.get_name(), "DaoBase");
        }

        #[ink::test]
        fn test_symbol() {
            let mut base = DaoBase::default();

            base.set_symbol("https://example.com/logo.jpg".to_string());

            let dbg_msg = format!("logo is {}", base.get_symbol());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_symbol(), "https://example.com/logo.jpg");
        }

        #[ink::test]
        fn test_synopsis() {
            let mut base = DaoBase::default();

            base.set_synopsis("This is DAO information".to_string());

            let dbg_msg = format!("name is {}", base.get_synopsis());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_synopsis(), "This is DAO information");
        }

        #[ink::test]
        fn test_all() {

            let accounts =ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

            let mut base = DaoBase::default();

            base.init_base("DaoBase".to_string(),  "This is DAO information".to_string(),"http://example.com/logo.jpg".to_string());

            let dbg_msg = format!("name is {}", base.get_name());
            ink_env::debug_println!("{}", &dbg_msg );

            assert_eq!(base.get_name(), "DaoBase");
            assert_eq!(base.get_symbol(), "http://example.com/logo.jpg");
            assert_eq!(base.get_synopsis(), "This is DAO information");
            assert_eq!(base.get_owner(), accounts.alice);

            let dbg_msg2 = format!("name is {:?}", base.get_owner());
            ink_env::debug_println!("{}", &dbg_msg2 );
        }
    }

}

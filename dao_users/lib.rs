#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_env::Environment;
extern crate alloc;
#[ink::contract]
mod dao_users {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };
    #[ink(storage)]
    pub struct DaoUsers {
       user_info:StorageHashMap<AccountId,User>,
       // user_referer:StorageHashMap<AccountId,AccountId>,
       code_user:StorageHashMap<[u8; 32],AccountId>,
       length:u128
    }
    impl DaoUsers {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                user_info:StorageHashMap::new(),
                code_user:StorageHashMap::new(),
                length:0,
            }
        }
        #[ink(message)]
        pub fn join(&mut self,invitation_code:[u8; 32],name:String,user_profile:String) -> Result<(), RandomReadErr> {
            // assert_eq!(self.length + 1 > self.length, true);
            let caller = self.env().caller();
            let user = self.user_info.get(&caller).unwrap().clone();
            assert_eq!(self.exists_user(caller),false);
            // let code = self.create_code();
            let code =  self.env().extension().fetch_random()?;

            self.code_user.insert(code,caller);
            let referer = if invitation_code.is_empty() { AccountId::default()} else { self.get_user_by_code(invitation_code) };
            let nickname = if name.is_empty() { String::default()} else {name };
            let profile = if user_profile.is_empty() { String::default()} else {user_profile };
            self.user_info.insert(caller, User{id:self.length + 1,nickname,profile,code,address:caller,referer,childs:Vec::new()});
            self.length += 1;
            if referer != AccountId::default() {
                self.insert_user_child(referer,caller);
            }
            Ok(())
        }
        #[ink(message)]
        pub fn get_user_referer(&self,user:AccountId) -> AccountId {
           let user_info : User =  self.user_info.get(&user).unwrap().clone();
            return user_info.referer;
        }
        #[ink(message)]
        pub fn exists_user(&self,user:AccountId) -> bool {
            let user_info = self.user_info.get(&user).unwrap().clone();
            return user_info.id != 0 ;
        }

        #[ink(message)]
        pub fn get_user_by_code(&self,invitation_code:[u8; 32]) -> AccountId {
            self.code_user.get(&invitation_code).unwrap().clone()
        }
        #[ink(message)]
        pub fn list_user(&self) -> Vec<User> {
            let mut user_vec = Vec::new();
            let mut iter = self.user_info.values();
            let mut user = iter.next();
            while user.is_some() {
                user_vec.push(user.unwrap().clone());
                user = iter.next();
            }
            user_vec
        }
        #[ink(message)]
        pub fn insert_user_child(&mut self,user:AccountId,child:AccountId) -> bool {
            let mut user_info = self.user_info.get_mut(&user).unwrap().clone();
            user_info.childs.push(child);
            true
        }
        #[ink(message)]
        pub fn set_nickname(&mut self,nickname:String) -> bool {
            let caller = self.env().caller();
            let mut user_info : User =  self.user_info.get_mut(&caller).unwrap().clone();
            user_info.nickname = nickname;
            true
        }
    }

}
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod dao_categeory{
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };

    #[ink(storage)]
    pub struct Daocategeory{
       owner:AccountId,
       index:u64,
       categeorys:StorageHashMap<u64,String>
    }
    impl  Daocategeory{
        #[ink(constructor)]
        pub fn new()->Self{
            Self{
                owner:Self::env().caller(),
                index:0,
                categeorys:StorageHashMap::new(),
            }
        }
        #[ink(message)]
        pub fn add_categeorys(&mut self, categeorys:String)->bool{
            self.categeorys.insert(self.index,categeorys);
            self.index +=1;
            true
        }
        #[ink(message)]
        pub fn  list_categeorys(&mut self )-> Vec<String>{
            let mut categeory_vec=Vec::new();
            let mut iter=self.categeorys.values();
            let mut categeory=iter.next();
            while categeory.is_some(){
                categeory_vec.push(categeory.unwrap().clone());
                categeory=iter.next();
            }
            categeory_vec

        }
        #[ink(message)]
        pub fn categeorys_by_indexs(&self ,index:u64)->String{
            self.categeorys.get(&index).unwrap().clone()
        }
    }
}

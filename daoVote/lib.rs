#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::daoVote::{
    DaoVote
};

#[ink::contract]
mod daoVote {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct VoteInfo{
        name:String,
        number:u128,//voter turnout
        vote:u128,//Number of votes cast


    }
  
    #[ink(storage)]
    pub struct DaoVote {
        owner:AccountId,
        //consignor:AccountId,
        name:String,
        index:u128,
        balance:StorageHashMap<AccountId,u128>, //id->balance
        list_vote:StorageHashMap<u64,String>,
        //weight grading (Tentative three grades)
        weightVotes:StorageHashMap<u128,u128>,
        vote:u128,
        erc20_instance:Erc20,


    }

    impl DaoVote {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut list=StorageHashMap::new();
            list.insert(1,String::from("a_currency_one_vote"));
            list.insert(2,String::from("one_man_one_vote"));
            list.insert(3,String::from("weight_vote"));
            list.insert(4,String::from("basic_quadratic_vote"));
            list.insert(5,String::from("weight_quadratic_vote"));
            Self { 
                owner:Self::env().caller(),
                name:String::default(),
                index:0,
                balance:StorageHashMap::new(),
                list_vote:list,
                weightVotes:StorageHashMap::new(),
                vote:0,
                erc20_instance: Default::default(),
             }
        }
        #[ink(constructor)]
        pub fn default() -> Self{
            Self::new()
        } 
        #[ink(message)]
        pub fn set_weight_vote(&mut self) {
           // let total_balance = Self::env().balance();
           let total_balance=self.balance.get(&self.env().caller()).unwrap().clone();
            self.balance.insert(self.env().caller(),total_balance);
            if total_balance<100{
                self.weightVotes.insert(total_balance,1);
            }
            else if total_balance>100&&total_balance<1000{
                self.weightVotes.insert(total_balance,2);
            }
            else if total_balance>1000{
                self.weightVotes.insert(total_balance,3);
            }
        }

        #[ink(message)]
        pub fn select_vote(&mut self,index:u128,name:String) {
            self.name=name;
            if let index=1{
                self.index+=1;
                self.vote=self.balance.get(&self.owner).unwrap().clone();
            }
            else if index==2{
                self.index+=1;
                self.vote+=self.index;
            }
            else if index==3{
                self.index+=1;
                self.vote+=self.weightVotes.get(&(self.balance.get(&self.owner).unwrap().clone())).unwrap().clone();

            }
            else if index==4{
                self.index+=1;
                self.vote+=(self.balance.get(&self.owner).unwrap().clone())^2/1;

            }
            else if index==5{
                self.index+=1;
                self.vote+=(self.weightVotes.get(&(self.balance.get(&self.owner).unwrap().clone()))).unwrap().clone()^2/1;

            }
            
        }

        #[ink(message)]
        pub fn select_vote_name(&self)->String{
            self.name.clone()
        }
        #[ink(message)]
        pub fn select_vote_info(&self)->VoteInfo{
            VoteInfo {
                name:self.name.clone(),
                number:self.index,
                vote:self.vote,
            } 
        }
        //add test balance
        #[ink(message)]
        pub fn set_balance(&mut self,balance:u128){
            let owner=self.env().caller();
            self.balance.insert(owner,balance);

        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn entrust_vote(&self,user:AccountId,value: Balance,erc20Addr:AccountId) -> bool {
            let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(erc20Addr);
            erc20_instance.approve(user,value);
            true
        }

         /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
         /// module and test functions are marked with a `#[test]` attribute.
         /// The below code is technically just normal Rust code.
         #[cfg(test)]
         mod tests {
         /// Imports all the definitions from the outer scope so we can use them here.
         use super::*;

         /// Imports `ink_lang` so we can use `#[ink::test]`.
         use ink_lang as ink;

         /// We test if the default constructor does its job.
         #[ink::test]
         fn default_works() {
             let daoVote = DaoVote::default();
             assert_eq!(daoVote.get(), false);
         }

         /// We test a simple use case of our contract.
         #[ink::test]
         fn it_works() {
             let mut daoVote = DaoVote::new(false);
             assert_eq!(daoVote.get(), false);
             daoVote.flip();
             assert_eq!(daoVote.get(), true);
        }

        
    
    }
}

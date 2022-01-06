#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::incomeCategory::{
    IncomeCategory,
};

use ink_lang as ink;

#[ink::contract]
mod incomeCategory {
    //use ink_prelude::vec::Vec;
    use erc20::Erc20;
    use daoVault::DaoVault;
    use alloc::string::String;
    use ink_prelude::vec::Vec;
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
    #[derive(Debug)]
    pub struct IncomeInfo {
       fee: u128,
       describe:String,
    }
    #[ink(storage)]
    pub struct IncomeCategory {
        treasury_vault:u128,
        dao_member_number:u128,
        super_manager:AccountId,
        erc20_instance:Erc20,
        vault_instance:DaoVault,
        category:StorageHashMap<String, IncomeInfo>,
        dao_category:StorageHashMap<u64,String>,
    }

    impl IncomeCategory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(treasury_vault:u128,super_manager:AccountId) -> Self {
            let mut list=StorageHashMap::new();
            list.insert(1,String::from("SonDao"));
            list.insert(2,String::from("independentDao"));
            list.insert(3,String::from("allianceDao"));
            
            Self { 
                treasury_vault,
                super_manager,
                dao_member_number:0,
                erc20_instance: Default::default(),
                vault_instance:Default::default(),
                category:StorageHashMap::new(),
                dao_category:list,

                 }
                 
        }
        #[ink(constructor)]
        pub fn default() -> Self{
            Self::new(Default::default(),Default::default())
        } 
        ///Create A Dao usage fee
        #[ink(message)]
        pub fn create_cost(&mut self, amount:u128, types:i32)->bool{
            let from = self.env().caller();
            //Specify the cost of creating different DAOs。（1:SonDao; 2:independentDao; 3:allianceDao）
            if let types=1{
                assert!(amount==100); 
                self.env().transfer(self.super_manager, amount);
            }
            else if let types=2{
                assert!(amount==200); 
                self.env().transfer(self.super_manager, amount);

            }
            else if types==3{
                assert!(amount==500); 
                self.env().transfer(self.super_manager, amount);
            }
            true

        }
          ///DCV treasury usage fee 
          #[ink(message)]
          pub fn using_vault (&mut self,amount:u128,vault_adrr:AccountId)->bool{
            // let from = self.env().caller();
            let adrr=self.get_vault_by_address(vault_adrr);
            let owner=adrr.value_owner();
            assert_eq!(amount==100,true); 
            self.env().transfer(owner, amount);
            true
          }
          ///The amount paid is determined by the number of DAO members
          #[ink(message)]
          pub fn member_number(&mut self,number:u128,amount:u128)->bool{
              let from = self.env().caller();
              if number >=100||number<1000{
                assert_eq!(amount==number*5,true);
                self.dao_member_number=number;
                self.env().transfer(self.super_manager, amount);
                
              }else if number>=1000{
                assert_eq!(amount==number*6,true);
                self.dao_member_number=number;
                self.env().transfer(self.super_manager, amount);
                
              }else{
                assert_eq!(amount==0,true);
                self.dao_member_number=number;
              }
              true
          }
          ///Distribution of fiscal revenue
         #[ink(message)]
         pub fn financial_allocation(&mut self,to:AccountId,addr:AccountId) ->bool{
             let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(addr);
             let from  = self.env().caller();
             let mut numbers=self.dao_member_number;
            // let mut money=(self.treasury_vault)*20/100/numbers;
             let mut money=((self.treasury_vault)*0.2 as u128 )as u64;
              erc20_instance.transfer_from(from, to, money.into() );
             true
         }
         #[ink(message)]
         pub fn get_vault_by_address(&self, address:AccountId) -> DaoVault {
            let  vault_instance: DaoVault = ink_env::call::FromAccountId::from_account_id(address);
            vault_instance
        }

        #[ink(message)]
        pub fn set_category(&mut self,name:String,fee:u128,info:String) -> bool {
            self.category.insert(name,
                IncomeInfo{
                    fee,
                    describe:info,
                }
            );
            true
        }

        #[ink(message)]
        pub fn get_category(&mut self,name:String) -> IncomeInfo {
           self.category.get(&name).unwrap().clone()
        }
        
        #[ink(message)]
        pub fn list_category(&self) -> Vec<IncomeInfo> {
            let mut category_vec = Vec::new();
            let mut iter = self.category.values();
            let mut category = iter.next();
            while category.is_some() {
                category_vec.push(category.unwrap().clone());
                category = iter.next();
            }
            category_vec
        }

    }

}

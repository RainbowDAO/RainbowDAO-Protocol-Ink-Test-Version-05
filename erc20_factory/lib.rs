
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::erc20_factory::{
    Erc20Factory,
};
#[ink::contract]
mod erc20_factory{
    use alloc::string::String;
    use erc20::Erc20;
    use ink_prelude::vec::Vec;
    use core::ptr::null;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    #[derive(scale::Encode,scale::Decode, Clone, SpreadLayout,PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    
     pub struct Info{
        owner:AccountId,
        index:u64,
        symbol:String,
        initial_supply:u64,
 
     }

   const RENT_VALUE: u128 =  1000 * 1_000_000_000_000;



    #[ink (storage)]
    pub struct Erc20Factory{
        owner:AccountId,
        index:u64,
        symbol:String,
        name:String,
        token:BTreeMap<u64,AccountId>,
        Infos:StorageHashMap<u64,Info>,

    }
    impl Erc20Factory {
        #[ink(constructor)]
        pub fn new()->Self{
            Self{
                owner:Default::default(),
                index:1,
                symbol:Default::default(),
                token:BTreeMap::new(),
                name:Default::default(),
                Infos:StorageHashMap::new(),
            }
        }
        #[ink(constructor)]
        pub fn default() -> Self{
            Self::new()
        } 
        #[ink(message)]
        pub fn mint_erc20(&mut self,erc20_hash: Hash,version: u32,name:String,symbol:String ,initial_supply:u64,adr: AccountId,decimals:u8)->bool{

            let salt=version.to_le_bytes();
            let instance_params=Erc20::new(initial_supply.into(),name.clone(),symbol.clone(),decimals,adr)
            .endowment(RENT_VALUE)
            .code_hash(erc20_hash)
            .salt_bytes(salt)
            .params();

            let instance_result=ink_env::instantiate_contract(&instance_params);
            let contract_addr=instance_result.expect("failed at instantiating the `ERC20` contract");
            //self.symbol=symbol.clone();
            //self.name=name.clone();
            //self.token.insert(self.index,contract_addr);
            self.Infos.insert(self.index,
                Info{
                    owner:adr,
                    index:self.index,
                    symbol,
                    initial_supply,
                   }
            );
            self.index+=1;
            true
            
        }

        #[ink(message)]
        pub fn get_Block(&self) -> Timestamp {
            self.env().block_timestamp()
        }

       #[ink(message)]
        pub fn get_transaction(&self,index:u64) -> Info {
            self.Infos.get(&index).unwrap().clone()
        }
    }
}
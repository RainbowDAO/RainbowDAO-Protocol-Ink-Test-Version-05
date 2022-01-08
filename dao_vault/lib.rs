#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
extern crate alloc;
pub use self::dao_vault::DaoVault;
#[ink::contract]
mod dao_vault {
    use alloc::string::String;
    use alloc::vec::Vec;
    use erc20::Erc20;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout,SpreadLayout},
    };

    #[derive(
        Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
        )]
        #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
        )]
        pub struct Transfer {
            transfer_id:u64,
            transfer_direction:u64,// 1: out 2 : in
            token_name: String,
            from_address:AccountId,
            to_address:AccountId,
            value: u64,
            transfer_time:u64,
        }

    #[derive(
        Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,Default
        )]
        #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
        )]
    pub struct TokenInfo {
        erc20: AccountId,
        symbol: String,
        name: String,
        balance: u64,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct DaoVault {
        value_manager:AccountId,
        vault_contract_address:AccountId,
        transfer_history:StorageHashMap<u64,Transfer>,
        tokens: StorageHashMap<AccountId, AccountId>,

    }

    #[ink(event)]
    pub struct AddVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }

    #[ink(event)]
    pub struct RemoveVaultTokenEvent {
        #[ink(topic)]
        token_address: AccountId,

    }


    #[ink(event)]
    pub struct DepositTokenEvent {

        #[ink(topic)]
        token_name:String,
        #[ink(topic)]
        from_address:AccountId,

        #[ink(topic)]
        value:u64,
    }


    #[ink(event)]
    pub struct WithdrawTokenEvent {
        #[ink(topic)]
        token_name:String,

        #[ink(topic)]
        to_address:AccountId,

        #[ink(topic)]
        value:u64,
    }

    impl DaoVault {
        #[ink(constructor)]
        pub fn new() -> Self {
            let contract_address = Self::env().account_id();
            Self { 
                value_manager:Self::env().caller(),
                vault_contract_address:contract_address,
                transfer_history:StorageHashMap::new(),
                tokens: StorageHashMap::default(),
            }
        }

        pub fn get_erc20_by_address(&self, address:AccountId) -> Erc20 {
            let  erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(address);
            erc20_instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        
        #[ink(message)]
        pub fn deposit(&mut self, erc_20_address:AccountId, from_address:AccountId,value:u64) -> bool {
            let to_address = self.vault_contract_address;
            if self.tokens.contains_key(&erc_20_address){
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);
                let token_name=(&erc_20).name();
                let transfer_result=erc_20.transfer_from(from_address,to_address,value.into());
                // if transfer_result == false {
                //     return false;
                // }
                let transfer_id:u64 = (self.transfer_history.len()+1).into();
                let transfer_time: u64 = self.env().block_timestamp();
                self.transfer_history.insert(transfer_id,
                    Transfer{
                        transfer_direction:2,// 1: out 2: in
                        token_name:token_name.clone(),
                        transfer_id:transfer_id,
                        from_address:from_address,
                        to_address:to_address,
                        value,
                        transfer_time});
            self.env().emit_event(DepositTokenEvent{
                token_name: token_name.clone(),
                from_address:from_address,
                value:value});
                true
        
            } else{
                false
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self,erc_20_address:AccountId,to_address:AccountId,value:u64) -> bool {
            let from_address = self.vault_contract_address;
            if self.tokens.contains_key(&erc_20_address) {
                let mut erc_20 = self.get_erc20_by_address(erc_20_address);
                let token_name=(&erc_20).name();
                let transfer_result=erc_20.transfer_from(from_address,to_address,value.into());
                // if transfer_result == false {
                //     return false;
                // }
                let transfer_id:u64 = (self.transfer_history.len()+1).into();

                let transfer_time: u64 = self.env().block_timestamp();

                self.transfer_history.insert(transfer_id,
                                             Transfer{
                                                 transfer_direction:1,// 1: out 2: in
                                                 token_name: token_name.clone(),
                                                 transfer_id:transfer_id,
                                                 from_address:from_address,
                                                 to_address:to_address,
                                                 value:value,
                                                 transfer_time:transfer_time});
                self.env().emit_event(WithdrawTokenEvent{
                    token_name: token_name.clone(),
                    to_address:to_address,
                    value:value,});

                true

            } else{
                false
            }
        }
    
        #[ink(message)]
        pub fn add_vault_token(&mut self,erc_20_address:AccountId) -> bool  {
           self.tokens.insert(erc_20_address,self.vault_contract_address);
           self.env().emit_event(AddVaultTokenEvent{
            token_address:erc_20_address,
            });
           true
        }
        
        #[ink(message)]
        pub fn remove_vault_token(&mut self,erc_20_address: AccountId) -> bool  {
            let contract_address=self.vault_contract_address;
            // for(erc_20_address, contract_address) in &self.tokens.into_iter(){
            //     self.tokens.remove(erc_20_address);
            // }
           // self.tokens.insert(erc_20_address,address(0));
            //self.tokens.remove(erc_20_address);
            self.env().emit_event(RemoveVaultTokenEvent{
                token_address:erc_20_address,
                });
            true
        }
        #[ink(message)]
        pub fn value_owner(&self) -> AccountId {
            self.value_manager.clone()
        }
    }
}
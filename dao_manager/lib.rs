#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::dao_manager::{
    DAOManager,
};

#[ink::contract]
mod dao_manager {
    use alloc::string::String;
    use dao_base::DaoBase;
    use erc20::Erc20;
    use template_manager::DAOTemplate;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use dao_vault::DaoVault;
    use dao_vote::DaoVote;
    use ink_storage::{
        // collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},   
    };

    const CONTRACT_INIT_BALANCE: u128 = 100 * 1000 * 1_000_000_000_000;


    /// DAO component instances
    #[derive(Debug, scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponents {
        pub base: Option<DaoBase>,
        pub erc20: Option<Erc20>,
        pub dao_vault: Option<DaoVault>,
        pub dao_vote: Option<DaoVote>,
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct DAOComponentAddrs {
        // base module contract's address
        pub base_addr: Option<AccountId>,
        // erc20 module contract's address
        pub erc20_addr: Option<AccountId>,
        pub vault_addr: Option<AccountId>,
        // vote module contract's address
        pub vote_addr: Option<AccountId>,
        
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct BaseParam {
        owner: AccountId,
        name: String,
        logo: String,
        desc: String,
    }

    /// DAO component instance addresses
    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct ERC20Param {
        owner: AccountId,
        name: String,
        symbol: String,
        total_supply: u64,
        decimals: u8,
    }

    #[derive(
    Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Default
    )]
    #[cfg_attr(
    feature = "std",
    derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct DAOInitParams {
        base: BaseParam,
        erc20: ERC20Param,
    }


    #[ink(storage)]
    pub struct DAOManager {
        init: bool,
        owner: AccountId,
        template: Option<DAOTemplate>,
        dao_id:u64,
        components: DAOComponents,
        component_addrs: DAOComponentAddrs,
    }

    impl DAOManager {
        /// Create a new dao
        #[ink(constructor)]
        pub fn new(owner:AccountId,dao_id:u64) -> Self {
            Self {
                init: false,
                owner,
                template:None,
                dao_id,
                components:DAOComponents {
                    base: None,
                    erc20:None,
                    dao_vault: None,
                    dao_vote: None,
                },
                component_addrs:DAOComponentAddrs{
                    base_addr:None,
                    erc20_addr:None,
                    vault_addr: None,
                    vote_addr: None,
                },
            }
        }

        /// Set the dao use which template
        #[ink(message)]
        pub fn set_template(&mut self, template: DAOTemplate) -> bool {
            assert_eq!(self.init, false);
            self.template = Some(template);
            true
        }
        #[ink(message)]
        pub fn  get_balance(&mut self) -> u128 {
            let total_balance = Self::env().balance();
            return total_balance;
        }
        #[ink(message)]
        pub fn  get_owner(&mut self) -> AccountId {
            self.owner
        }

        /// Initialize Dao and generate various
        #[ink(message)]
        pub fn  init_by_params(&mut self, params: DAOInitParams, salt: Vec<u8>) -> bool {
            assert_eq!(self.init, false);
            assert_eq!(self.template.is_some(), true);
            let owner = self.env().caller();
            assert_eq!(owner == self.owner, true);
            let components_hash_map = self.template.as_ref().unwrap().components.clone();
            let base_code_hash = components_hash_map.get("BASE");
            let erc20_code_hash = components_hash_map.get("ERC20");
            let vault_code_hash = components_hash_map.get("VAULT");
            let vote_code_hash = components_hash_map.get("VOTE");
            self._init_base(base_code_hash, params.base, &salt);
            self._init_erc20(erc20_code_hash, params.erc20, &salt);
            self._init_vault(vault_code_hash, &salt);
            self._init_vote(vote_code_hash, &salt);
            self.init = true;
            true
        }

        /// init base
        fn _init_base(&mut self, base_code_hash: Option<&Hash>,
                      param: BaseParam, salt: &Vec<u8>) -> bool {
            if base_code_hash.is_none() {
                return true;
            }
            let base_code_hash = base_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // instance base
            // let salt = version.to_le_bytes();
            let instance_params = DaoBase::new()
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(base_code_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Base` contract");
            let mut contract_instance: DaoBase = ink_env::call::FromAccountId::from_account_id(contract_addr);
            contract_instance.init_base(param.name, param.logo, param.desc);

            self.components.base = Some(contract_instance);
            self.component_addrs.base_addr = Some(contract_addr);

            true
        }
        
        fn _init_erc20(&mut self, erc20_code_hash: Option<&Hash>,
            param: ERC20Param, salt: &Vec<u8>) -> bool {
            if erc20_code_hash.is_none() {
                return true;
            }
            let erc20_code_hash = erc20_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            // let vault_addr = self.component_addrs.vault_addr.unwrap();
            let erc20_instance_params = Erc20::new(0,param.name, param.symbol,
                 param.decimals, Self::env().account_id())
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .params();
            let erc20_init_result = ink_env::instantiate_contract(&erc20_instance_params);
            let erc20_addr = erc20_init_result.expect("failed at instantiating the `Erc20` contract");
            let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(erc20_addr);

            erc20_instance.mint_token_by_owner(param.owner, param.total_supply);
            erc20_instance.transfer_owner(param.owner);

            self.components.erc20 = Some(erc20_instance);
            self.component_addrs.erc20_addr = Some(erc20_addr);
            true
        }

        fn _init_vault(&mut self, vault_code_hash: Option<&Hash>, salt: &Vec<u8>) -> bool {
            if vault_code_hash.is_none() {
                return true;
            }
            let vault_code_hash = vault_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            let vault_instance_params = DaoVault::new()
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(vault_code_hash)
                .salt_bytes(salt)
                .params();
            let vault_init_result = ink_env::instantiate_contract(&vault_instance_params);
            let vault_addr = vault_init_result.expect("failed at instantiating the `Org` contract");
            let vault_instance: DaoVault = ink_env::call::FromAccountId::from_account_id(vault_addr);
            self.components.dao_vault = Some(vault_instance);
            self.component_addrs.vault_addr = Some(vault_addr);
            true
        }

        /// init vote
        fn _init_vote(&mut self, vote_code_hash: Option<&Hash>, salt: &Vec<u8>) -> bool {
            if vote_code_hash.is_none() {
                return true;
            }
            let vote_code_hash = vote_code_hash.unwrap().clone();
            let total_balance = Self::env().balance();
            assert!(total_balance > CONTRACT_INIT_BALANCE, "not enough unit to instance contract");
            let vote_instance_params = DaoVote::new()
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(vote_code_hash)
                .salt_bytes(salt)
                .params();
            let vote_init_result = ink_env::instantiate_contract(&vote_instance_params);
            let vote_addr = vote_init_result.expect("failed at instantiating the `Vote` contract");
            let vote_instance = ink_env::call::FromAccountId::from_account_id(vote_addr);
            self.components.dao_vote = Some(vote_instance);
            self.component_addrs.vote_addr = Some(vote_addr);
            true
        }

    }

}

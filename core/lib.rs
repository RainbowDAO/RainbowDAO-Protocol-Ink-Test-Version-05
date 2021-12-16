#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use ink_lang as ink;
pub use self::core::{
    Core
};
#[ink::contract]
mod core {
    use alloc::string::String;
    // use ink_storage::Lazy;
    use role_manage::RoleManage;
    use route_manage::RouteManage;
    use authority_management::AuthorityManagement;
    const DAO_INIT_BALANCE: u128 = 1_000_000_000_000;

    #[ink(storage)]
    pub struct Core {
        owner:AccountId,
        role_manage: Option<RoleManage>,
        role_manage_addr: Option<AccountId>,
        route_manage: Option<RouteManage>,
        route_manage_addr: Option<AccountId>,
        authority_manage:Option<AuthorityManagement>,
        authority_manage_addr:Option<AccountId>,
    }

    impl Core {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                owner:Self::env().caller(),
                role_manage : None,
                role_manage_addr : None,
                route_manage : None,
                route_manage_addr : None,
                authority_manage : None,
                authority_manage_addr : None,
            };
            instance
        }
        #[ink(message)]
        pub fn add_role(&mut self, name: String) {
            // self.role_manage.add_role(name);
            self.role_manage.as_mut().unwrap().add_role(name);
        }
        #[ink(message)]
        pub fn add_authority(&mut self, name: String) {
            // self.privilege_manage.add_privilege(name);
            self.authority_manage.as_mut().unwrap().add_authority(name);
        }
        #[ink(message)]
        pub fn add_route(&mut self, name: String,addr:AccountId) {
            // self.route_manage.add_route(name,v);
            self.route_manage.as_mut().unwrap().add_route(name,addr);
        }

        #[ink(message)]
        pub fn init(&mut self, version: u32,role_code_hash: Hash,privilege_code_hash: Hash,route_code_hash: Hash) -> bool {
            let salt = version.to_le_bytes();
            let role_manage = RoleManage::new()
                .endowment(DAO_INIT_BALANCE)
                .code_hash(role_code_hash)
                .salt_bytes(salt)
                .params();
            let init_role_result = ink_env::instantiate_contract(&role_manage);
            let role_manage_addr = init_role_result.expect("failed at instantiating the `roleManager` contract");
            let role_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.role_manage = Some(role_contract_instance);
            self.role_manage_addr = Some(role_manage_addr);

            let authority_management = AuthorityManagement::new()
                .endowment(DAO_INIT_BALANCE)
                .code_hash(privilege_code_hash)
                .salt_bytes(salt)
                .params();
            let init_authority_result = ink_env::instantiate_contract(&authority_management);
            let authority_management_addr = init_authority_result.expect("failed at instantiating the `TemplateManager` contract");
            let authority_contract_instance = ink_env::call::FromAccountId::from_account_id(authority_management_addr);
            self.authority_manage = Some(authority_contract_instance);
            self.authority_manage_addr = Some(authority_management_addr);

            let route_manage = RouteManage::new()
                .endowment(DAO_INIT_BALANCE)
                .code_hash(route_code_hash)
                .salt_bytes(salt)
                .params();
            let init_route_result = ink_env::instantiate_contract(&route_manage);
            let route_manage_addr = init_route_result.expect("failed at instantiating the `TemplateManager` contract");
            let route_contract_instance = ink_env::call::FromAccountId::from_account_id(role_manage_addr);
            self.route_manage = Some(route_contract_instance);
            self.route_manage_addr = Some(route_manage_addr);
            true
        }
        #[ink(message)]
        pub fn get_Balance(&self) -> u128 {
            return Self::env().balance();
        }



    }

    
}

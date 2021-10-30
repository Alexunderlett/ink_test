#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::test2::{
    Test2,
};
use ink_lang as ink;
#[ink::contract]
mod test2 {
    use test::Test;

    #[ink(storage)]
    pub struct Test2 {
        value: bool,
        test_instance: Test,
        new_address: AccountId,
    }
    impl Test2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value: false,
                test_instance: Default::default(),
                new_address: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        ///第一种实例化
        ///param  addr: test合约地址
        #[ink(message)]
        pub fn test_instance(&mut self,addr: AccountId) -> i32 {
            let vault_instance: Test = ink_env::call::FromAccountId::from_account_id(addr);
            let a = vault_instance.get_value();
            a
        }

        ///第二种实例化
        ///param  addr:test合约地址
        #[ink(message)]
        pub fn test_instance2(&mut self,account_id: Test) -> i32 {
            let a = account_id.get_value();
            a
        }

        ///创建新合约
        #[ink(message)]
        pub fn creat_new_contract(&mut self,init_value: i32,test_hash: Hash) -> bool {
            let total_balance = Self::env().balance();
            // instance base
            let num: i32 = 1;
            let salt = num.to_le_bytes();
            let instance_params = Test::new(init_value)
                .endowment(total_balance /2)
                .code_hash(test_hash)
                .salt_bytes(salt)
                .params();
            let test_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = test_result.expect("failed at instantiating the `Base` contract");
            let contract_instance: Test = ink_env::call::FromAccountId::from_account_id(contract_addr);

            self.test_instance = contract_instance;
            self.new_address = contract_addr;

            true
        }

        ///获取新合约的地址
        #[ink(message)]
        pub fn get_new_address(&self) -> AccountId {
            self.new_address
        }

        ///读取新合约的属性
        #[ink(message)]
        pub fn get_test_value(&self) -> i32 {
            self.test_instance.get_value()
        }

        ///查看合约的剩余租金
        #[ink(message)]
        pub fn get_total_balance(&self) -> u128 {
            let total_balance = Self::env().balance();
            total_balance
        }
    }
}
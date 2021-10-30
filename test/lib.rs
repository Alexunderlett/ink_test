#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::test::{
    Test,
};
use ink_lang as ink;
#[ink::contract]
mod test {

    #[ink(storage)]
    #[derive(Default)]
    pub struct Test {
        index: i32,
    }
    impl Test {
        #[ink(constructor)]
        pub fn new(index: i32) -> Self {
            Self {
                index,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        
        #[ink(message)]
        pub fn get_value(&self) -> i32 {
            self.index
        }
    }
}
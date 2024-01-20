use ethers::core::rand::thread_rng;
use ethers::signers::LocalWallet;
use crate::account::MyWallet;

pub fn generate_random_account() -> MyWallet {
    MyWallet(LocalWallet::new(&mut thread_rng()))
}

use crate::account::MyWallet;
use ethers::core::rand::thread_rng;
use ethers::signers::LocalWallet;

pub fn generate_random_account() -> MyWallet {
    MyWallet(LocalWallet::new(&mut thread_rng()))
}

#[cfg(test)]
mod test {
    use crate::account::keypair::generate_random_account;

    #[test]
    fn test_gen_random_account() {
        let my_wallet = generate_random_account();
        println!("addr: {:?}", my_wallet.get_address());
        println!("privkey: {:?}", my_wallet.get_private_key());
    }
}

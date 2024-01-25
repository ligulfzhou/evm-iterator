use crate::error::MyResult;
use crate::evm::account::GenAccount;
use crate::evm::my_wallet::MyWallet;
use ethers::core::rand::thread_rng;
use ethers::signers::LocalWallet;

pub struct RandomAccountGenerator;

impl GenAccount for RandomAccountGenerator {
    fn generate_account(&mut self) -> MyResult<MyWallet> {
        Ok(MyWallet(LocalWallet::new(&mut thread_rng())))
    }
}

#[cfg(test)]
mod test {
    use crate::evm::account::keypair::RandomAccountGenerator;
    use crate::evm::account::GenAccount;

    #[test]
    fn test_gen_random_account() {
        let mut generator = RandomAccountGenerator;
        let my_wallet = generator.generate_account();
        assert!(my_wallet.is_ok());
        let my_wallet = my_wallet.unwrap();
        // println!("addr: {:?}", my_wallet.get_address());
        assert_eq!(my_wallet.get_address().len(), 42);
        // println!("privkey: {:?}", my_wallet.get_private_key());
        assert_eq!(my_wallet.get_private_key().len(), 63);
    }
}

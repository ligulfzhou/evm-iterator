use crate::evm::my_wallet::MyWallet;
pub mod keypair;
pub mod mnemonic;

pub trait GenAccount {
    fn generate_account(&mut self) -> anyhow::Result<MyWallet>;
}

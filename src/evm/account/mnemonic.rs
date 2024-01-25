use crate::error::MyResult;
use crate::evm::account::GenAccount;
use crate::evm::my_wallet::MyWallet;
use ethers::core::rand;
use ethers::prelude::coins_bip39::Mnemonic;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};

pub struct MnemonicAccountGenerator {
    mnemonic: String,
    index: u32,
}

impl MnemonicAccountGenerator {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::<English>::new(&mut rng).to_phrase();

        Self { mnemonic, index: 0 }
    }

    pub fn regenerate_mnemonic(&mut self) {
        let mut rng = rand::thread_rng();
        self.mnemonic = Mnemonic::<English>::new(&mut rng).to_phrase();
        self.index = 0;
    }

    pub fn gen_from_mnemonic(&mut self) -> MyResult<MyWallet> {
        if self.index > 10 {
            self.regenerate_mnemonic();
        }
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(self.mnemonic.as_str())
            .index(self.index)?
            .build()?;

        Ok(MyWallet(wallet))
    }
}

impl GenAccount for MnemonicAccountGenerator {
    fn generate_account(&mut self) -> MyResult<MyWallet> {
        if self.index > 10 {
            self.regenerate_mnemonic();
        }
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(self.mnemonic.as_str())
            .index(self.index)?
            .build()?;

        Ok(MyWallet(wallet))
    }
}

#[cfg(test)]
mod test {
    use crate::evm::account::mnemonic::MnemonicAccountGenerator;
    use crate::evm::account::GenAccount;

    #[tokio::test]
    async fn test() {
        let mut generator = MnemonicAccountGenerator::new();
        assert!(generator.generate_account().is_ok());
    }
}

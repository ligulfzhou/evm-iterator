use crate::account::MyWallet;
use crate::error::MyResult;
use ethers::core::rand;
use ethers::prelude::coins_bip39::Mnemonic;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};

pub fn generate_mnemonic() -> MyResult<String> {
    let mut rng = rand::thread_rng();
    Ok(Mnemonic::<English>::new(&mut rng).to_phrase())
}

pub fn from_mnemonic(mnemonic: &str, index: u32) -> MyResult<MyWallet> {
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .index(index)?
        .build()?;

    Ok(MyWallet(wallet))
}

#[cfg(test)]
mod tests {
    use crate::account::mnemonic::generate_mnemonic;

    #[test]
    fn test_generate_mnemonic_code() {
        assert!(generate_mnemonic().is_ok())
    }

    #[test]
    fn test_from_mnemonic() {}
}

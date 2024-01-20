use crate::error::MyResult;
use ethers::core::rand;
use ethers::signers::{coins_bip39::English, LocalWallet, MnemonicBuilder};
use crate::account::MyWallet;

pub fn generate_mnemonic() -> MyResult<LocalWallet> {
    let mut rng = rand::thread_rng();
    Ok(MnemonicBuilder::<English>::default().build_random(&mut rng)?)
}

pub fn from_mnemonic(mnemonic: &str, index: u32) -> MyResult<MyWallet> {
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .index(index)?
        .build()?;

    Ok(MyWallet(wallet))
}


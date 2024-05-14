use ethers::providers::ProviderError;
use ethers::signers::WalletError;
use thiserror::Error;

#[warn(dead_code)]
pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("ProviderError: {:?}", .0)]
    ProviderError(#[from] ProviderError),

    #[error("WalletError: {:?}", .0)]
    WalletError(#[from] WalletError),
}

use ethers::providers::ProviderError;
use ethers::signers::WalletError;
use thiserror::Error;
pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug, Error)]
pub enum MyError {
    /// 登陆相关
    // #[error("登陆失败")]
    // LoginFail,

    // #[error("密码错误")]
    // LoginFailForPasswordIsWrong,

    // #[error("未登陆")]
    // NotAuthorized,
    #[error("ProviderError: {:?}", .0)]
    ProviderError(#[from] ProviderError),

    #[error("WalletError: {:?}", .0)]
    WalletError(#[from] WalletError),
}

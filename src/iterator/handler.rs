use crate::config::config::EvmConfig;
use crate::error::MyResult;
use crate::evm::my_wallet::MyWallet;

#[derive(Debug)]
pub struct EvmHandler {
    config: EvmConfig,
}

impl EvmHandler {
    pub fn new(config: EvmConfig) -> Self {
        Self { config }
    }

    pub async fn check_balance(&self, account: MyWallet) -> MyResult<()> {
        // check eth balance
        account.check_eth_balance(&self.config).await?;

        // check erc20 balance
        account.check_erc20_balance(&self.config).await?;

        Ok(())
    }
}

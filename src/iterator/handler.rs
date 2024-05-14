use crate::config::config::EvmConfig;
use crate::evm::my_wallet::MyWallet;

#[derive(Debug)]
pub struct EvmHandler {
    config: EvmConfig,
}

impl EvmHandler {
    pub fn new(config: EvmConfig) -> Self {
        Self { config }
    }

    pub async fn check_balance(&self, account: MyWallet) -> anyhow::Result<()> {
        // check eth balance
        let _ = account
            .check_eth_balance(&self.config)
            .await
            .map_err(|_| {});

        // check erc20 balance
        let _ = account
            .check_erc20_balance(&self.config)
            .await
            .map_err(|_| {});

        Ok(())
    }
}

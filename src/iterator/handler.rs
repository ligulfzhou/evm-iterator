use std::time::Duration;

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

    pub async fn check_balance(&self, account: MyWallet, interval: i32) -> anyhow::Result<()> {
        // check eth balance
        let _ = account
            .check_eth_balance(&self.config)
            .await
            .map_err(|_| {});

        tokio::time::sleep(Duration::new(interval as u64, 0)).await;

        // check erc20 balance
        let _ = account
            .check_erc20_balance(&self.config, interval)
            .await
            .map_err(|_| {});

        Ok(())
    }
}

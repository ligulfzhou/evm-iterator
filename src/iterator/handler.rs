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
        for erc20 in self.config.erc20s.iter() {
            println!(
                "check erc20#{:?} {:?} balance of: {:?}",
                erc20.name,
                erc20.contract,
                account.get_address()
            );
        }

        Ok(())
    }
}

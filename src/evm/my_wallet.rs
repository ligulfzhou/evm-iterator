use crate::config::config::EvmConfig;
use crate::error::MyResult;
use ethers::core::rand;
use ethers::core::rand::prelude::SliceRandom;
use ethers::prelude::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;

#[derive(Clone)]
pub struct MyWallet(pub LocalWallet);

impl MyWallet {
    pub fn get_private_key(&self) -> String {
        self.0
            .signer()
            .to_bytes()
            .iter()
            .map(|&i| format!("{:X}", i))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn get_h160_address(&self) -> Address {
        self.0.address()
    }
    pub fn get_address(&self) -> String {
        format!("{:#x}", self.0.address())
    }
}

impl MyWallet {
    pub async fn check_eth_balance(&self, config: &EvmConfig) -> MyResult<()> {
        let rpc = config
            .rpcs
            .choose(&mut rand::thread_rng())
            .expect("failed to pick up a rpc");

        let provider: Provider<Http> =
            Provider::<Http>::try_from(rpc).expect("create provider from url failed");

        let balance = provider.get_balance(self.get_h160_address(), None).await?;

        println!(
            "check eth_balance of: {:?}, {:?}",
            self.get_address(),
            balance
        );
        if balance > 0.into() {
            // to transfer
            println!("balance > 0, transfer to {:}", config.to);
        }

        Ok(())
    }

    pub async fn check_erc20_balance(&self, config: &EvmConfig, index: i32) -> MyResult<()> {
        let rpc = config
            .rpcs
            .choose(&mut rand::thread_rng())
            .expect("failed to choose a rpc");

        let provider: Provider<Http> =
            Provider::<Http>::try_from(rpc).expect("create provider from url failed");

        let balance = provider.get_balance(self.get_h160_address(), None).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_check_eth_balance() {}

    #[test]
    fn test_check_erc20_balance() {}
}

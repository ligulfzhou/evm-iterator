use ethers::core::rand;
use ethers::core::rand::prelude::SliceRandom;
use ethers::prelude::Http;
use crate::account::MyWallet;
use crate::config::config::EvmConfig;
use ethers::providers::Provider;


pub trait BalanceChecker {
    async fn check_balance(&self, account: MyWallet);
}

pub struct EvmHandler {
    config: EvmConfig,
}

impl EvmHandler {
    pub fn new(config: EvmConfig) -> Self {
        Self {
            config
        }
    }

    pub fn get_web3_cli(&self) {
    }
}

impl BalanceChecker for EvmHandler {
    async fn check_balance(&self, account: MyWallet) {
        // check eth balance
        let rpc= self.config.rpcs.choose(&mut rand::thread_rng()).expect("failed to choose a rpc");
        let provider = Provider::<Http>::try_from(rpc).expect("create provider from url failed");

        println!("check eth_balance of: {:?}", account.get_address());

        // check erc20 balance
        for erc20 in self.config.erc20s.iter() {
            println!("check erc20#{:?} {:?} balance of: {:?}", erc20.name, erc20.contract, account.get_address());
        }
    }
}

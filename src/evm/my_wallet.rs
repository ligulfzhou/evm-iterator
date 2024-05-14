use crate::config::config::EvmConfig;
use ethers::{
    contract::abigen,
    core::rand::{self, seq::SliceRandom},
    middleware::signer::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest},
};
use serde_json::Value;
use std::sync::Arc;

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

    pub fn get_account(&self) -> LocalWallet {
        self.0.clone()
    }
}

impl From<LocalWallet> for MyWallet {
    fn from(value: LocalWallet) -> Self {
        Self(value)
    }
}

impl MyWallet {
    pub async fn check_eth_balance(&self, config: &EvmConfig) -> anyhow::Result<()> {
        let provider = {
            let rpc = config
                .rpcs
                .choose(&mut rand::thread_rng())
                .expect("failed to pick up a rpc");

            Provider::<Http>::try_from(rpc).expect("create provider from url failed")
        };

        let signer = Arc::new(SignerMiddleware::new(provider, self.get_account()));

        let balance = signer.get_balance(self.get_h160_address(), None).await?;
        dbg!(&balance);

        if balance <= 0.into() {
            return Ok(());
        }

        // to transfer
        println!("balance > 0, transfer to {:}", config.to);
        let gas_price = signer.get_gas_price().await?;
        let value = balance - gas_price * 100;

        let to_address = config
            .to
            .clone()
            .parse::<Address>()
            .expect("parse to address failed");
        let tx = TransactionRequest::pay(to_address, value).from(self.get_h160_address());

        signer
            .send_transaction(tx, None)
            .await?
            .log_msg("Pending transfer")
            .await?;
        Ok(())
    }

    pub async fn check_erc20_balance(&self, config: &EvmConfig) -> anyhow::Result<()> {
        let provider = {
            let rpc = config
                .rpcs
                .choose(&mut rand::thread_rng())
                .expect("failed to pick up a rpc");

            Provider::<Http>::try_from(rpc).expect("create provider from url failed")
        };
        let signer = Arc::new(SignerMiddleware::new(provider, self.get_account()));

        abigen!(Erc20Contract, "./src/assets/erc20.json");

        for erc20 in config.erc20s.iter() {
            let contract_address = erc20
                .contract
                .clone()
                .parse::<Address>()
                .expect("erc20 contract address not valid");

            let contract = Erc20Contract::new(contract_address, signer.clone());

            let balance = contract
                .balance_of(self.get_h160_address())
                .call()
                .await
                .unwrap_or(0.into());

            if balance == 0.into() {
                continue;
            }

            let gas_price = signer.get_gas_price().await?;
            let value = balance - gas_price * 100;

            let to_address = config
                .to
                .parse::<Address>()
                .expect("should work to parse address");

            let tx = contract.transfer(to_address, value);
            let mined_tx = tx.send().await?.await?;

            dbg!(&mined_tx);

            // Extract the tx hash for printing
            let json_str = serde_json::to_string(&mined_tx).expect("");
            let json: Value = serde_json::from_str(&json_str).expect("");

            if let Some(transaction_hash) = json["transactionHash"].as_str() {
                println!("Transaction Hash: {}", transaction_hash);
            } else {
                println!("Transaction Hash not found");
            }
        }

        Ok(())
    }
}

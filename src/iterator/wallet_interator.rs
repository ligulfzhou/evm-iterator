use crate::evm::account::GenAccount;
use crate::evm::my_wallet::MyWallet;
use crate::iterator::handler::EvmHandler;
use std::time::Duration;
use tokio::time::sleep;

pub struct AccountGenerator {
    pub observers: Vec<EvmHandler>,
    pub generators: Vec<Box<dyn GenAccount>>,
}

impl AccountGenerator {
    pub fn new() -> Self {
        Self {
            observers: vec![],
            generators: vec![],
        }
    }

    pub async fn start_generating_accounts(&mut self, interval: i32) -> anyhow::Result<()> {
        loop {
            let accounts = self
                .generators
                .iter_mut()
                .map(|generatorgener| generatorgener.generate_account())
                .filter_map(|item| match item {
                    Ok(new_account) => Some(new_account),
                    Err(_) => None,
                })
                .collect::<Vec<MyWallet>>();

            for account in accounts.into_iter() {
                self.notify_observers(account, interval).await?;
            }
        }
    }

    pub fn add_generator(&mut self, generator: Box<dyn GenAccount>) {
        self.generators.push(generator);
    }

    pub fn remove_generator(&mut self, generator: Box<dyn GenAccount>) {
        self.generators.retain(|o| !std::ptr::eq(o, &generator));
    }

    pub fn add_observer(&mut self, observer: EvmHandler) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, observer: EvmHandler) {
        self.observers.retain(|o| !std::ptr::eq(o, &observer));
    }

    pub async fn notify_observers(&self, account: MyWallet, interval: i32) -> anyhow::Result<()> {
        for observer in self.observers.iter() {
            observer.check_balance(account.clone(), interval).await?;
        }

        Ok(())
    }
}

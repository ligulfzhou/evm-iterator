use crate::account::keypair::generate_random_account;
use crate::account::MyWallet;
use crate::iter::evm_handler::BalanceChecker;
use std::time::Duration;

pub trait Subject {
    fn add_observer(&mut self, observer: Box<dyn BalanceChecker>);
    fn remove_observer(&mut self, observer: Box<dyn BalanceChecker>);
    async fn notify_observers(&self, account: MyWallet);
}

pub struct AccountGenerator {
    observers: Vec<Box<dyn BalanceChecker>>,
}

impl AccountGenerator {
    pub fn new() -> Self {
        Self { observers: vec![] }
    }

    pub async fn start_generating_accounts(&self) {
        loop {
            tokio::time::sleep(Duration::new(5, 0)).await;

            let new_account = generate_random_account();
            println!("get new account: {:?}", new_account.get_address());
            self.notify_observers(new_account).await;
        }
    }
}

impl Subject for AccountGenerator {
    fn add_observer(&mut self, observer: Box<dyn BalanceChecker>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Box<dyn BalanceChecker>) {
        // Box::eq
        self.observers.retain(|o| !std::ptr::eq(o, &observer));
    }

    async fn notify_observers(&self, account: MyWallet) {
        for observer in self.observers.iter() {
            observer.check_balance(account.clone()).await;
        }
    }
}

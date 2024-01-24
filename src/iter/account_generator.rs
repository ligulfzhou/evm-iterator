use std::thread::sleep;
use std::time::Duration;
use crate::account::keypair::generate_random_account;
use crate::account::MyWallet;
use crate::iter::evm_handler::BalanceChecker;

pub trait Subject {
    fn add_observer(&mut self, observer: Box<dyn BalanceChecker>);
    fn remove_observer(&mut self, observer: Box<dyn BalanceChecker>);
    fn notify_observers(&self, account: MyWallet);
}

pub struct AccountGenerator {
    observers: Vec<Box<dyn BalanceChecker>>,
}

impl AccountGenerator {
    pub fn new() -> Self {
        Self { observers: vec![] }
    }

    pub fn start_generating_accounts(&mut self) {
        loop {
            sleep(Duration::new(5, 0));

            let new_account = generate_random_account();
            self.notify_observers(new_account);
        }
    }
}

impl Subject for AccountGenerator {
    fn add_observer(&mut self, observer: Box<dyn BalanceChecker>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Box<dyn BalanceChecker>) {
        // self.observers.retain(|o| !Box::eq(o, &observer));
        todo!()
    }

    fn notify_observers(&self, account: MyWallet) {
        self.observers.iter().for_each(|observer| {
            observer.check_balance(account.clone())
        })
    }
}
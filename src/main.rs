use crate::error::MyResult;
use crate::evm::account::keypair::RandomAccountGenerator;
use crate::evm::account::mnemonic::MnemonicAccountGenerator;
use crate::iterator::handler::EvmHandler;
use crate::iterator::wallet_interator::AccountGenerator;

mod config;
mod error;
mod evm;
mod iterator;

#[tokio::main]
async fn main() -> MyResult<()> {
    let evms_config = config::config::load_evm_configs()?;

    let handlers = evms_config
        .evms
        .into_iter()
        .map(EvmHandler::new)
        .collect::<Vec<EvmHandler>>();

    let mut account_generator = AccountGenerator::new();
    handlers
        .into_iter()
        .for_each(|handler| account_generator.add_observer(handler));

    let random_account_generator = RandomAccountGenerator;
    let mnemonic_account_generator = MnemonicAccountGenerator::new();
    account_generator.add_generator(Box::new(random_account_generator));
    account_generator.add_generator(Box::new(mnemonic_account_generator));

    let _ = account_generator.start_generating_accounts(evms_config.interval).await;

    Ok(())
}

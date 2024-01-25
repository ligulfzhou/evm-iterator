use crate::error::MyResult;
use crate::evm::account::keypair::RandomAccountGenerator;
use crate::iterator::handler::EvmHandler;
use crate::iterator::wallet_interator::AccountGenerator;

// mod account;
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
        .map(|evm| EvmHandler::new(evm))
        .collect::<Vec<EvmHandler>>();

    let mut account_generator = AccountGenerator::new();
    for handler in handlers {
        account_generator.add_observer(handler);
    }

    let random_generator = RandomAccountGenerator;
    account_generator.add_generator(Box::new(random_generator));

    account_generator.start_generating_accounts().await;

    Ok(())
}

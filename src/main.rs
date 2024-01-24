use crate::error::MyResult;
use crate::iter::account_generator::{AccountGenerator, Subject};
use crate::iter::evm_handler::EvmHandler;

mod account;
mod config;
mod error;
mod iter;

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
        account_generator.add_observer(Box::new(handler));
    }

    account_generator.start_generating_accounts().await;

    Ok(())
}

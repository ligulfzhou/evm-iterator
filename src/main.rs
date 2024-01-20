use crate::error::MyResult;

mod account;
mod config;
mod error;
mod evms;

#[tokio::main]
async fn main() -> MyResult<()> {
    let evms_config = config::config::load_evm_configs()?;

    println!("configs: {:?}", evms_config);
    println!("Hello, world!");

    Ok(())
}

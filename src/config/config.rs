use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Erc20 {
    pub name: String,
    pub contract: String,
}

#[derive(Debug, Deserialize)]
pub struct EvmConfig {
    pub name: String,
    pub rpcs: Vec<String>,
    pub erc20s: Vec<Erc20>,
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct EvmConfigs {
    pub evms: Vec<EvmConfig>,
    pub interval: i32,
}

pub fn load_evm_configs() -> anyhow::Result<EvmConfigs> {
    let current_dir = Path::new(file!())
        .parent()
        .expect("Failed to get current directory");

    let file_path = Path::new(&current_dir).join("evms.yaml");
    let mut file = File::open(file_path.as_path()).expect("Failed to open evms.yaml file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Failed to read file");

    let evm_configs: EvmConfigs =
        serde_yaml::from_str(&file_content).expect("Failed to deserialize");

    Ok(evm_configs)
}

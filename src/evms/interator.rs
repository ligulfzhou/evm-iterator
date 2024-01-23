use crate::config::config::EvmConfig;

#[derive(Debug)]
pub struct Iterator {
    pub config: EvmConfig,
}

impl Iterator {
    pub fn new(config: EvmConfig) -> Self {
        Self { config }
    }

    pub fn start(&self) {
        loop {}
    }
}

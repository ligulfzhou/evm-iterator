use ethers::prelude::Http;
use web3::api::Eth;
use web3::Web3;

struct EthUtil {
    pub web3_http_host: String,
}

impl EthUtil {
    pub fn new(host: String) -> Self {
        Self {
            web3_http_host: host,
        }
    }
}

pub struct Erc20 {
    pub contract_address: String,
}

impl Erc20 {
    pub fn new(contract_address: String) -> Self {
        Self { contract_address }
    }

    pub fn get_balance(&self, account: String) {}
}

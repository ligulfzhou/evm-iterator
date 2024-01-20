use ethers::prelude::LocalWallet;
use web3::signing::Key;

pub mod keypair;
pub mod mnemonic;

struct MyWallet(LocalWallet);

impl MyWallet {
    pub fn get_private_key(&self) -> String {
        self.0
            .signer()
            .to_bytes()
            .iter()
            .map(|&i| format!("{:X}", i))
            .collect::<Vec<String>>()
            .join("")
    }
}


#[cfg(test)]
mod test {
    use crate::account::MyWallet;
    use crate::account::mnemonic::{from_mnemonic};
    use ethers:: types::H160;

    #[tokio::test]
    async fn test() {
        let wallet = from_mnemonic("fatal brass black survey crucial assist timber pattern execute sister illegal trade friend rival main", 0).unwrap();
        assert_eq!(
            "0x7c47f87b1cbfef6e6a2159851ec16541b8f17536"
                .parse::<H160>()
                .unwrap(),
            wallet.0.address()
        );
    }

    #[test]
    fn test_private_key() {
        let wallet = from_mnemonic("fatal brass black survey crucial assist timber pattern execute sister illegal trade friend rival main", 0).unwrap();
        assert_eq!(
            "0x7c47f87b1cbfef6e6a2159851ec16541b8f17536"
                .parse::<H160>()
                .unwrap(),
        );

        let mywallet = MyWallet(wallet.clone());
        assert_eq!(
            mywallet.get_private_key(),
            "51C581A24244425924AEF2F64FCAD5F1A53C2B153918AD2B65A5070759C".to_string()
        );
    }
}

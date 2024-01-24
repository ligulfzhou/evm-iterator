use ethers::signers::{LocalWallet, Signer};

pub mod keypair;
pub mod mnemonic;

#[derive(Clone)]
pub struct MyWallet(LocalWallet);

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

    pub fn get_address(&self) -> String {
        format!("{:#x}", self.0.address())
    }
}

#[cfg(test)]
mod test {
    use crate::account::mnemonic::from_mnemonic;

    #[tokio::test]
    async fn test() {
        let my_wallet = from_mnemonic("fatal brass black survey crucial assist timber pattern execute sister illegal trade friend rival main", 0).unwrap();
        println!("mywallet_address: {:?}", my_wallet.get_address());

        assert_eq!(
            "0x7c47f87b1cbfef6e6a2159851ec16541b8f17536",
            my_wallet.get_address()
        );
    }

    #[test]
    fn test_private_key() {
        let my_wallet = from_mnemonic("fatal brass black survey crucial assist timber pattern execute sister illegal trade friend rival main", 0).unwrap();

        assert_eq!(
            my_wallet.get_private_key(),
            "51C581A24244425924AEF2F64FCAD5F1A53C2B153918AD2B65A5070759C".to_string()
        );

        let my_wallet = from_mnemonic("fatal brass black survey crucial assist timber pattern execute sister illegal trade friend rival main", 1).unwrap();
        assert_eq!(
            "89E9ADC4494993B73691D5B226B31E2BEB8B10C9E3DC6D975F9ED1E5A7BFF891".to_string(),
            my_wallet.get_private_key()
        );
    }
}

use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Identity {
    wallet: LocalWallet,
}

impl Identity {
    /// Create a new identity with a random wallet.
    pub fn new_random() -> Self {
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = macroquad_toolkit::rng::gen_range(0u8, 255u8);
        }
        let wallet = LocalWallet::from_bytes(&bytes).expect("Valid random bytes");
        Self { wallet }
    }

    /// Create a new identity with a hardcoded dev wallet.
    /// Wallet address: 0x...
    pub fn new_dev() -> Self {
        // Safe only for development!
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let wallet = private_key.parse::<LocalWallet>().expect("Invalid dev key");
        Self { wallet }
    }

    /// Get the wallet address as a string.
    pub fn address(&self) -> String {
        format!("{:?}", self.wallet.address())
    }

    /// Sign a message.
    pub async fn sign(&self, message: &str) -> String {
        self.wallet.sign_message(message).await.unwrap().to_string()
    }
}

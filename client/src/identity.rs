#[cfg(not(target_arch = "wasm32"))]
use ethers::signers::{LocalWallet, Signer};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct Identity {
    wallet: LocalWallet,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct Identity {
    address: String,
}

impl Identity {
    /// Create a new identity with a random wallet.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    pub fn new_random() -> Self {
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = macroquad_toolkit::rng::gen_range(0u8, 255u8);
        }
        let wallet = LocalWallet::from_bytes(&bytes).expect("Valid random bytes");
        Self { wallet }
    }

    /// Create a new identity with the deterministic dev wallet address.
    #[cfg(target_arch = "wasm32")]
    #[allow(dead_code)]
    pub fn new_random() -> Self {
        Self::new_dev()
    }

    /// Create a new identity with a hardcoded dev wallet.
    /// Wallet address: 0x...
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_dev() -> Self {
        // Safe only for development!
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let wallet = private_key.parse::<LocalWallet>().expect("Invalid dev key");
        Self { wallet }
    }

    /// Create a new identity with a hardcoded dev wallet address.
    #[cfg(target_arch = "wasm32")]
    pub fn new_dev() -> Self {
        Self {
            address: "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf".to_string(),
        }
    }

    /// Get the wallet address as a string.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn address(&self) -> String {
        format!("{:?}", self.wallet.address())
    }

    /// Get the wallet address as a string.
    #[cfg(target_arch = "wasm32")]
    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// Sign a message.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn sign(&self, message: &str) -> String {
        self.wallet.sign_message(message).await.unwrap().to_string()
    }

    /// WebGL builds need browser wallet integration for real signatures.
    #[cfg(target_arch = "wasm32")]
    pub async fn sign(&self, message: &str) -> String {
        format!("wasm-dev-signature:{}", message)
    }
}

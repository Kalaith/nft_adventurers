//! Authentication handlers.

use axum::Json;
use crate::models::{ChallengeResponse, VerifyRequest, SessionResponse};
use uuid::Uuid;

/// Generate auth challenge for wallet signing.
pub async fn challenge() -> Json<ChallengeResponse> {
    let nonce = Uuid::new_v4().to_string();
    let message = format!("Sign this message to authenticate with NFT Adventurers: {}", nonce);

    Json(ChallengeResponse { nonce, message })
}

use ethers::types::{Address, Signature};
use std::str::FromStr;


/// Verify wallet signature and create session.
pub async fn verify(Json(request): Json<VerifyRequest>) -> Json<SessionResponse> {
    let message = format!("Sign this message to authenticate with NFT Adventurers: {}", request.nonce);
    
    let signature = match Signature::from_str(&request.signature) {
        Ok(s) => s,
        Err(_) => return Json(SessionResponse {
            success: false,
            wallet_address: None,
            session_token: None,
        }),
    };

    let address = match Address::from_str(&request.wallet_address) {
        Ok(a) => a,
        Err(_) => return Json(SessionResponse {
            success: false,
            wallet_address: None,
            session_token: None,
        }),
    };

    // Recover address from signature
    match signature.recover(message) {
        Ok(recovered) => {
            if recovered == address {
                let session_token = Uuid::new_v4().to_string();
                println!("Auth: wallet={} verified", request.wallet_address);
                Json(SessionResponse {
                    success: true,
                    wallet_address: Some(request.wallet_address),
                    session_token: Some(session_token),
                })
            } else {
                println!("Auth: wallet={} signature invalid", request.wallet_address);
                Json(SessionResponse {
                    success: false,
                    wallet_address: None,
                    session_token: None,
                })
            }
        }
        Err(e) => {
            println!("Auth: verification error: {}", e);
            Json(SessionResponse {
                success: false,
                wallet_address: None,
                session_token: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::signers::{LocalWallet, Signer};
    use rand::thread_rng;

    #[tokio::test]
    async fn test_verify_signature_success() {
        let wallet = LocalWallet::new(&mut thread_rng());
        let nonce = Uuid::new_v4().to_string();
        let message = format!("Sign this message to authenticate with NFT Adventurers: {}", nonce);
        let signature = wallet.sign_message(&message).await.unwrap();
        
        // Ethers 2.0 / LocalWallet might need explicit use of ethers::prelude or traits
        
        let request = VerifyRequest {
            wallet_address: format!("{:?}", wallet.address()),
            signature: signature.to_string(),
            nonce,
        };

        let response = verify(Json(request)).await;
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_verify_signature_failure() {
        let wallet = LocalWallet::new(&mut thread_rng());
        let wallet2 = LocalWallet::new(&mut thread_rng()); // Different wallet
        let nonce = Uuid::new_v4().to_string();
        let message = format!("Sign this message to authenticate with NFT Adventurers: {}", nonce);
        let signature = wallet2.sign_message(&message).await.unwrap();
        
        let request = VerifyRequest {
            wallet_address: format!("{:?}", wallet.address()),
            signature: signature.to_string(),
            nonce,
        };

        let response = verify(Json(request)).await;
        assert!(!response.0.success);
    }
}

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

/// Verify wallet signature and create session.
/// For dev mode: accepts any signature and creates a session.
pub async fn verify(Json(request): Json<VerifyRequest>) -> Json<SessionResponse> {
    // TODO: In production, verify signature using ethers
    // For now, accept any request in dev mode
    
    let session_token = Uuid::new_v4().to_string();
    
    println!(
        "Auth: wallet={} verified (dev mode)",
        request.wallet_address
    );

    Json(SessionResponse {
        success: true,
        wallet_address: Some(request.wallet_address),
        session_token: Some(session_token),
    })
}

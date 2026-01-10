//! API request/response models.

use serde::{Deserialize, Serialize};

/// Auth challenge response.
#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub nonce: String,
    pub message: String,
}

/// Auth verify request.
#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub wallet_address: String,
    pub signature: String,
    pub nonce: String,
}

/// Session response after verification.
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub success: bool,
    pub wallet_address: Option<String>,
    pub session_token: Option<String>,
}

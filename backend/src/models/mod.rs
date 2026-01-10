//! API request/response models.

use serde::{Deserialize, Serialize};

/// Generic API response.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

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

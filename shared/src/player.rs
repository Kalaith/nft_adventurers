//! Player data types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Player profile identified by wallet address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub wallet_address: String,
    pub created_at: DateTime<Utc>,
}

impl Player {
    /// Create a new player.
    pub fn new(wallet_address: String) -> Self {
        Self {
            wallet_address,
            created_at: Utc::now(),
        }
    }
}

/// Full player data returned from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub player: Player,
    pub adventurers: Vec<crate::Adventurer>,
    pub items: Vec<crate::Item>,
    pub consumables: Vec<crate::Consumable>,
    pub hold: crate::Hold,
    pub active_missions: Vec<crate::ActiveMission>,
}

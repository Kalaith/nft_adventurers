//! Hold building types.
//! Now data-driven - building data comes from backend.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Echo from a dead adventurer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Echo {
    pub adventurer_name: String,
    pub class: String,
    pub death_mission: String,
    pub buff_description: String,
}

/// Player's hold (base).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hold {
    pub owner: String,
    /// Building type_key -> level mapping
    pub buildings: HashMap<String, u32>,
    pub echoes: Vec<Echo>,
    pub total_feats: u32,
}

impl Hold {
    /// Create a new hold with starter building.
    pub fn new(owner: String) -> Self {
        let mut buildings = HashMap::new();
        buildings.insert("hearth".to_string(), 1);

        Self {
            owner,
            buildings,
            echoes: Vec::new(),
            total_feats: 0,
        }
    }

    /// Get building level (0 if not built).
    pub fn building_level(&self, building_key: &str) -> u32 {
        *self.buildings.get(building_key).unwrap_or(&0)
    }
}

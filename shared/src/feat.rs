//! Feat system types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source of a feat (how it was earned).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatSource {
    Kill { enemy_type: String, count: u32 },
    Survive { mission_type: String },
    Craft,
    Inherit { from_adventurer: String },
}

/// Bonuses granted by a feat.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeatBonuses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_vs_type: Option<(String, f32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_bonus: Option<(String, i32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xp_bonus: Option<f32>,
}

/// A feat recorded in the Legendary Ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feat {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: String,
    pub name: String,
    pub source: FeatSource,
    pub bonuses: FeatBonuses,
    pub timestamp: DateTime<Utc>,
}

impl Feat {
    /// Create a new feat.
    pub fn new(
        entity_id: Uuid,
        entity_type: &str,
        name: String,
        source: FeatSource,
        bonuses: FeatBonuses,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            entity_id,
            entity_type: entity_type.to_string(),
            name,
            source,
            bonuses,
            timestamp: Utc::now(),
        }
    }
}

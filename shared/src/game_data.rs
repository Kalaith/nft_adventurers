//! Game data types that come from backend.

use serde::{Deserialize, Serialize};

/// Mission type data loaded from backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionTypeData {
    pub type_key: String,
    pub display_name: String,
    pub description: String,
    pub duration_seconds: u64,
    pub permadeath_chance: f32,
    pub reward_multiplier: f32,
    pub difficulty_class: u32,
    pub icon_key: String,
}

/// Item type data loaded from backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeData {
    pub type_key: String,
    pub display_name: String,
    pub icon_key: String,
    pub equip_slot: String,
    pub base_damage: Option<u32>,
    pub base_defense: Option<u32>,
    pub base_healing: Option<u32>,
}

/// Class type data loaded from backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassTypeData {
    pub type_key: String,
    pub display_name: String,
    pub portrait_key: String,
    pub base_str: u32,
    pub base_dex: u32,
    pub base_int: u32,
    pub base_con: u32,
    pub base_cha: u32,
    #[serde(default)]
    pub unlock_level: u32,
    #[serde(default)]
    pub cost: u32,
}

/// Consumable type data loaded from backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableTypeData {
    pub type_key: String,
    pub display_name: String,
    pub description: String,
    pub icon_key: String,
    pub effect_type: String,
    pub effect_value: f32,
}

/// Building type data loaded from backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingTypeData {
    pub type_key: String,
    pub display_name: String,
    pub description: String,
    pub icon_key: String,
    pub xp_bonus_per_level: f32,
}

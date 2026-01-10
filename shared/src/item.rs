//! Item types with feat history.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Feat;

/// Item rarity tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

/// Equipment slot for items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipSlot {
    Weapon,
    Armor,
    Accessory,
}

impl EquipSlot {
    /// Parse from string.
    pub fn from_str(s: &str) -> Self {
        match s {
            "weapon" => EquipSlot::Weapon,
            "armor" => EquipSlot::Armor,
            "accessory" => EquipSlot::Accessory,
            _ => EquipSlot::Weapon, // default
        }
    }
}

/// Base stats for items.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healing: Option<u32>,
}

/// An item entity with feat history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub owner: String,
    /// Item type key (e.g., "sword", "staff") - references item_types table.
    pub type_key: String,
    /// Equipment slot for this item.
    pub equip_slot: EquipSlot,
    pub current_name: String,
    pub rarity: Rarity,
    pub base_stats: ItemStats,
    pub equipped_by: Option<Uuid>,
    pub feats: Vec<Feat>,
}

impl Item {
    /// Create a new item with explicit stats.
    pub fn new(
        owner: String,
        type_key: String,
        equip_slot: EquipSlot,
        name: String,
        rarity: Rarity,
        base_stats: ItemStats,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner,
            type_key,
            equip_slot,
            current_name: name,
            rarity,
            base_stats,
            equipped_by: None,
            feats: Vec::new(),
        }
    }

    /// Create starter sword for new players.
    pub fn starter_sword(owner: String) -> Self {
        Self::new(
            owner,
            "sword".to_string(),
            EquipSlot::Weapon,
            "Rusty Sword".to_string(),
            Rarity::Common,
            ItemStats {
                damage: Some(10),
                ..Default::default()
            },
        )
    }

    /// Check if item is equipped.
    pub fn is_equipped(&self) -> bool {
        self.equipped_by.is_some()
    }
}

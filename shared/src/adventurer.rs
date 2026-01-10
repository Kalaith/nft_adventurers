//! Adventurer class types available in v1.0.
//! Now data-driven - actual class data comes from backend.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Current status of an adventurer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdventurerStatus {
    Healthy,
    Injured { hours_remaining: u32 },
    OnMission { mission_id: Uuid },
    Dead,
}

/// Base stats for adventurers (D&D inspired).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub str_: u32,
    pub dex: u32,
    pub int: u32,
    pub con: u32,
    pub cha: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            str_: 10,
            dex: 10,
            int: 10,
            con: 10,
            cha: 10,
        }
    }
}

/// Equipment slots for an adventurer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Equipment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory: Option<Uuid>,
}

impl Equipment {
    pub fn slot_empty(&self, slot: crate::EquipSlot) -> bool {
        match slot {
            crate::EquipSlot::Weapon => self.weapon.is_none(),
            crate::EquipSlot::Armor => self.armor.is_none(),
            crate::EquipSlot::Accessory => self.accessory.is_none(),
        }
    }

    pub fn equip(&mut self, slot: crate::EquipSlot, item_id: Uuid) -> Option<Uuid> {
        match slot {
            crate::EquipSlot::Weapon => self.weapon.replace(item_id),
            crate::EquipSlot::Armor => self.armor.replace(item_id),
            crate::EquipSlot::Accessory => self.accessory.replace(item_id),
        }
    }

    pub fn unequip(&mut self, slot: crate::EquipSlot) -> Option<Uuid> {
        match slot {
            crate::EquipSlot::Weapon => self.weapon.take(),
            crate::EquipSlot::Armor => self.armor.take(),
            crate::EquipSlot::Accessory => self.accessory.take(),
        }
    }
}

/// An adventurer entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adventurer {
    pub id: Uuid,
    pub owner: String,
    pub name: String,
    /// Class type key - references class_types table
    pub class_key: String,
    pub level: u32,
    pub xp: u32,
    pub stats: Stats,
    pub skills: Vec<String>,
    pub status: AdventurerStatus,
    pub equipment: Equipment,
}

impl Adventurer {
    /// Create a new adventurer with provided stats.
    pub fn new(owner: String, name: String, class_key: String, stats: Stats) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner,
            name,
            class_key,
            level: 1,
            xp: 0,
            stats,
            skills: Vec::new(),
            status: AdventurerStatus::Healthy,
            equipment: Equipment::default(),
        }
    }

    /// Check if adventurer can go on a mission.
    pub fn is_available(&self) -> bool {
        matches!(self.status, AdventurerStatus::Healthy)
    }
}

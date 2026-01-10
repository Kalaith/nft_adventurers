//! Consumable items that can be used during missions.
//! Now data-driven - actual consumable data comes from backend.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A consumable item stack in the player's inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    pub id: Uuid,
    /// Type key - references consumable_types table
    pub type_key: String,
    pub quantity: u32,
}

impl Consumable {
    /// Create a new consumable stack.
    pub fn new(type_key: String, quantity: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            type_key,
            quantity,
        }
    }

    /// Add to the stack.
    pub fn add(&mut self, amount: u32) {
        self.quantity = self.quantity.saturating_add(amount);
    }

    /// Use from the stack. Returns true if successful.
    pub fn use_one(&mut self) -> bool {
        if self.quantity > 0 {
            self.quantity -= 1;
            true
        } else {
            false
        }
    }

    /// Check if stack is empty.
    pub fn is_empty(&self) -> bool {
        self.quantity == 0
    }
}

/// Effect modifiers from consumables applied to a mission.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsumableEffects {
    pub hp_restore: f32,
    pub fire_resistance: f32,
    pub permadeath_reduction: f32,
}

impl ConsumableEffects {
    /// Apply a consumable effect by type and value.
    pub fn apply_effect(&mut self, effect_type: &str, effect_value: f32) {
        match effect_type {
            "hp_restore" => self.hp_restore = (self.hp_restore + effect_value).min(1.0),
            "fire_resist" => self.fire_resistance = (self.fire_resistance + effect_value).min(1.0),
            "permadeath_reduce" => self.permadeath_reduction = (self.permadeath_reduction + effect_value).min(1.0),
            _ => {}
        }
    }
}

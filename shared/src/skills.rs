//! Skill tree definitions for v1.0.

use serde::{Deserialize, Serialize};

/// A skill node in a skill tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: u32, // 1-5, unlocks sequentially
    pub effect: SkillEffect,
}

/// Possible skill effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillEffect {
    StatBonus { stat: String, amount: i32 },
    DamageBonus { percent: i32 },
    HealingBonus { percent: i32 },
    DeathResist { percent: i32 },
    XpBonus { percent: i32 },
}

/// Skill tree for a class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    pub class: String,
    pub nodes: Vec<SkillNode>,
}

impl SkillTree {
    /// Create Warrior skill tree.
    pub fn warrior() -> Self {
        Self {
            class: "warrior".to_string(),
            nodes: vec![
                SkillNode {
                    id: "warrior_1".to_string(),
                    name: "Iron Will".to_string(),
                    description: "+2 CON".to_string(),
                    tier: 1,
                    effect: SkillEffect::StatBonus {
                        stat: "con".to_string(),
                        amount: 2,
                    },
                },
                SkillNode {
                    id: "warrior_2".to_string(),
                    name: "Weapon Mastery".to_string(),
                    description: "+10% damage".to_string(),
                    tier: 2,
                    effect: SkillEffect::DamageBonus { percent: 10 },
                },
                SkillNode {
                    id: "warrior_3".to_string(),
                    name: "Last Stand".to_string(),
                    description: "-10% death chance".to_string(),
                    tier: 3,
                    effect: SkillEffect::DeathResist { percent: 10 },
                },
                SkillNode {
                    id: "warrior_4".to_string(),
                    name: "Battle Rage".to_string(),
                    description: "+20% damage".to_string(),
                    tier: 4,
                    effect: SkillEffect::DamageBonus { percent: 20 },
                },
                SkillNode {
                    id: "warrior_5".to_string(),
                    name: "Indomitable".to_string(),
                    description: "-20% death chance".to_string(),
                    tier: 5,
                    effect: SkillEffect::DeathResist { percent: 20 },
                },
            ],
        }
    }

    /// Create Mage skill tree.
    pub fn mage() -> Self {
        Self {
            class: "mage".to_string(),
            nodes: vec![
                SkillNode {
                    id: "mage_1".to_string(),
                    name: "Arcane Focus".to_string(),
                    description: "+2 INT".to_string(),
                    tier: 1,
                    effect: SkillEffect::StatBonus {
                        stat: "int".to_string(),
                        amount: 2,
                    },
                },
                SkillNode {
                    id: "mage_2".to_string(),
                    name: "Spell Amplifier".to_string(),
                    description: "+15% damage".to_string(),
                    tier: 2,
                    effect: SkillEffect::DamageBonus { percent: 15 },
                },
                SkillNode {
                    id: "mage_3".to_string(),
                    name: "Quick Study".to_string(),
                    description: "+15% XP gain".to_string(),
                    tier: 3,
                    effect: SkillEffect::XpBonus { percent: 15 },
                },
                SkillNode {
                    id: "mage_4".to_string(),
                    name: "Arcane Shield".to_string(),
                    description: "-15% death chance".to_string(),
                    tier: 4,
                    effect: SkillEffect::DeathResist { percent: 15 },
                },
                SkillNode {
                    id: "mage_5".to_string(),
                    name: "Spell Mastery".to_string(),
                    description: "+30% damage".to_string(),
                    tier: 5,
                    effect: SkillEffect::DamageBonus { percent: 30 },
                },
            ],
        }
    }

    /// Create Cleric skill tree.
    pub fn cleric() -> Self {
        Self {
            class: "cleric".to_string(),
            nodes: vec![
                SkillNode {
                    id: "cleric_1".to_string(),
                    name: "Divine Grace".to_string(),
                    description: "+2 CHA".to_string(),
                    tier: 1,
                    effect: SkillEffect::StatBonus {
                        stat: "cha".to_string(),
                        amount: 2,
                    },
                },
                SkillNode {
                    id: "cleric_2".to_string(),
                    name: "Healing Touch".to_string(),
                    description: "+20% healing".to_string(),
                    tier: 2,
                    effect: SkillEffect::HealingBonus { percent: 20 },
                },
                SkillNode {
                    id: "cleric_3".to_string(),
                    name: "Divine Protection".to_string(),
                    description: "-15% death (party)".to_string(),
                    tier: 3,
                    effect: SkillEffect::DeathResist { percent: 15 },
                },
                SkillNode {
                    id: "cleric_4".to_string(),
                    name: "Sanctified Aura".to_string(),
                    description: "+10% XP gain".to_string(),
                    tier: 4,
                    effect: SkillEffect::XpBonus { percent: 10 },
                },
                SkillNode {
                    id: "cleric_5".to_string(),
                    name: "Resurrection".to_string(),
                    description: "-30% death chance".to_string(),
                    tier: 5,
                    effect: SkillEffect::DeathResist { percent: 30 },
                },
            ],
        }
    }
}


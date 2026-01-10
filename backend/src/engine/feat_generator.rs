//! Feat generation logic.

use rand::Rng;
use shared::{Feat, FeatBonuses, FeatSource};
use uuid::Uuid;

/// Enemy types for feat generation.
const ENEMY_TYPES: &[&str] = &["goblin", "orc", "skeleton", "dragon"];

/// Prefixes based on enemy type.
fn get_prefix(enemy_type: &str) -> &'static str {
    match enemy_type {
        "goblin" => "Gob",
        "orc" => "Orc",
        "skeleton" => "Bone",
        "dragon" => "Dragon",
        _ => "Foe",
    }
}

/// Suffixes based on kill count tier.
fn get_suffix(kill_count: u32) -> &'static str {
    match kill_count {
        0..=4 => "Bane",
        5..=9 => "Hunter",
        10..=24 => "Slayer",
        25..=49 => "Vanquisher",
        _ => "Doom",
    }
}

/// Generate a feat name from kills.
pub fn generate_feat_name(enemy_type: &str, kill_count: u32) -> String {
    format!("{}-{}", get_prefix(enemy_type), get_suffix(kill_count))
}

/// Generate feat bonuses based on kills.
pub fn generate_feat_bonuses(enemy_type: &str, kill_count: u32) -> FeatBonuses {
    let damage_bonus = (kill_count as f32 * 2.0).min(50.0);

    FeatBonuses {
        damage_vs_type: Some((enemy_type.to_string(), damage_bonus)),
        stat_bonus: None,
        xp_bonus: None,
    }
}

/// Generate a new feat for an entity.
pub fn generate_kill_feat(
    entity_id: Uuid,
    entity_type: &str,
    enemy_type: &str,
    kill_count: u32,
) -> Feat {
    let name = generate_feat_name(enemy_type, kill_count);
    let bonuses = generate_feat_bonuses(enemy_type, kill_count);
    let source = FeatSource::Kill {
        enemy_type: enemy_type.to_string(),
        count: kill_count,
    };

    Feat::new(entity_id, entity_type, name, source, bonuses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feat_name_generation() {
        assert_eq!(generate_feat_name("goblin", 3), "Gob-Bane");
        assert_eq!(generate_feat_name("dragon", 15), "Dragon-Slayer");
        assert_eq!(generate_feat_name("orc", 50), "Orc-Doom");
    }
}

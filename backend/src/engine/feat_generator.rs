use shared::{Feat, FeatBonuses, FeatSource};
use uuid::Uuid;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct SuffixRule {
    min_kills: u32,
    suffix: String,
}

#[derive(Debug, Deserialize)]
struct FeatConfig {
    enemy_types: Vec<String>,
    prefixes: HashMap<String, String>,
    suffixes: Vec<SuffixRule>,
    bonus_multiplier: f32,
    max_bonus: f32,
}

static CONFIG: OnceLock<FeatConfig> = OnceLock::new();

fn get_config() -> &'static FeatConfig {
    CONFIG.get_or_init(|| {
        let json = include_str!("../../assets/feat_config.json");
        serde_json::from_str(json).expect("Failed to parse feat_config.json")
    })
}

/// Prefixes based on enemy type.
fn get_prefix(enemy_type: &str) -> String {
    let config = get_config();
    config.prefixes.get(enemy_type)
        .cloned()
        .or_else(|| config.prefixes.get("default").cloned())
        .unwrap_or_else(|| "Foe".to_string())
}

/// Suffixes based on kill count tier.
fn get_suffix(kill_count: u32) -> String {
    let config = get_config();
    // Find the rule with the highest min_kills that is <= kill_count
    config.suffixes.iter()
        .filter(|rule| kill_count >= rule.min_kills)
        .max_by_key(|rule| rule.min_kills)
        .map(|rule| rule.suffix.clone())
        .unwrap_or_else(|| "Bane".to_string())
}

/// Generate a feat name from kills.
pub fn generate_feat_name(enemy_type: &str, kill_count: u32) -> String {
    format!("{}-{}", get_prefix(enemy_type), get_suffix(kill_count))
}

/// Generate feat bonuses based on kills.
pub fn generate_feat_bonuses(enemy_type: &str, kill_count: u32) -> FeatBonuses {
    let config = get_config();
    let damage_bonus = (kill_count as f32 * config.bonus_multiplier).min(config.max_bonus);

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
        assert_eq!(generate_feat_name("unknown", 5), "Foe-Hunter");
    }
}

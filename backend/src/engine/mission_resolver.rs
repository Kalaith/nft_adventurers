//! Mission resolution logic.

use rand::Rng;
use shared::{ActiveMission, Adventurer, MissionResult, MissionType};

/// Roll a D20.
fn roll_d20() -> u32 {
    rand::thread_rng().gen_range(1..=20)
}

/// Calculate party power from adventurers.
pub fn calculate_party_power(party: &[&Adventurer]) -> u32 {
    party.iter().map(|a| adventurer_power(a)).sum()
}

/// Calculate individual adventurer power.
fn adventurer_power(adventurer: &Adventurer) -> u32 {
    let stats = &adventurer.stats;
    let base = stats.str_ + stats.dex + stats.int + stats.con + stats.cha;
    let level_bonus = adventurer.level * 2;
    base + level_bonus
}

/// Resolve a mission and determine outcome.
/// Creates its own RNG internally to avoid Send issues with axum handlers.
pub fn resolve_mission(
    mission: &ActiveMission,
    party: &[&Adventurer],
) -> MissionResult {
    let mut rng = rand::thread_rng();
    
    let party_power = calculate_party_power(party);
    let roll = roll_d20();
    let dc = mission.mission_type.difficulty_class();

    // Check success: roll + power modifier vs DC
    let power_modifier = (party_power / 10) as u32;
    let total = roll + power_modifier;
    let success = total >= dc;

    // Determine deaths on failure
    let mut deaths = Vec::new();
    if !success {
        let death_chance = mission.mission_type.permadeath_chance();
        for adventurer in party {
            let death_roll: f32 = rng.gen();
            if death_roll < death_chance {
                deaths.push(adventurer.id);
            }
        }
    }

    // Calculate XP
    let base_xp = match mission.mission_type {
        MissionType::QuickSkirmish => 50,
        MissionType::DungeonCrawl => 150,
        MissionType::BossRaid => 500,
    };
    let xp_gained = if success {
        (base_xp as f32 * mission.mission_type.reward_multiplier()) as u32
    } else {
        base_xp / 2
    };

    MissionResult {
        mission_id: mission.id,
        success,
        xp_gained,
        feats_earned: Vec::new(),
        deaths,
        loot: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{AdventurerStatus, Class, Stats};
    use uuid::Uuid;

    fn test_adventurer() -> Adventurer {
        Adventurer {
            id: Uuid::new_v4(),
            owner: "test".to_string(),
            name: "Test Hero".to_string(),
            class: Class::Warrior,
            level: 1,
            xp: 0,
            stats: Stats::default(),
            skills: Vec::new(),
            status: AdventurerStatus::Healthy,
        }
    }

    #[test]
    fn test_party_power() {
        let adventurer = test_adventurer();
        let party = vec![&adventurer];
        let power = calculate_party_power(&party);
        assert!(power > 0);
    }
}

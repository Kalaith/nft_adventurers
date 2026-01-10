//! Hold management handlers.

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::queries;
use crate::AppState;

/// Request for upgrading a building.
#[derive(Debug, Deserialize)]
pub struct UpgradeBuildingRequest {
    pub wallet_address: String,
    pub building: String,
}

/// Response for building operations.
#[derive(Debug, Serialize)]
pub struct UpgradeBuildingResponse {
    pub success: bool,
    pub message: String,
    pub new_level: u32,
}

/// Request for unlocking a skill.
#[derive(Debug, Deserialize)]
pub struct UnlockSkillRequest {
    pub wallet_address: String,
    pub adventurer_id: String,
    pub skill_id: String,
}

/// Response for skill operations.
#[derive(Debug, Serialize)]
pub struct UnlockSkillResponse {
    pub success: bool,
    pub message: String,
}

/// Upgrade a building in the player's hold.
pub async fn upgrade_building(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpgradeBuildingRequest>,
) -> Json<UpgradeBuildingResponse> {
    let pool = &state.db.pool;

    let building_key = request.building.to_lowercase();

    // Get current hold
    let hold = match queries::get_hold(pool, &request.wallet_address).await {
        Ok(h) => h,
        Err(_) => {
            return Json(UpgradeBuildingResponse {
                success: false,
                message: "Failed to get hold".to_string(),
                new_level: 0,
            });
        }
    };

    let current_level = hold.building_level(&building_key);
    let max_level = 5;

    if current_level >= max_level {
        return Json(UpgradeBuildingResponse {
            success: false,
            message: "Building already at max level".to_string(),
            new_level: current_level,
        });
    }

    let new_level = current_level + 1;

    // Check cost
    if let Some(building_data) = queries::get_building_type_data(pool, &building_key).await.unwrap_or(None) {
        let scaling_factor = building_data.cost_scaling.powi(current_level as i32);
        let gold_cost = (building_data.base_cost_gold as f32 * scaling_factor) as u32;
        let lumber_cost = (building_data.base_cost_lumber as f32 * scaling_factor) as u32;
        let stone_cost = (building_data.base_cost_stone as f32 * scaling_factor) as u32;

        if let Err(_) = queries::spend_resources(pool, &request.wallet_address, gold_cost, lumber_cost, stone_cost).await {
             return Json(UpgradeBuildingResponse {
                success: false,
                message: format!("Insufficient resources. Need {} gold, {} lumber, {} stone.", gold_cost, lumber_cost, stone_cost),
                new_level: current_level,
            });
        }
    }

    // Update building level in database
    match queries::upgrade_building(pool, &request.wallet_address, &building_key, new_level).await {
        Ok(_) => {
            println!("Building upgraded: {} -> Lv.{}", building_key, new_level);
            Json(UpgradeBuildingResponse {
                success: true,
                message: format!("{} upgraded to level {}!", building_key, new_level),
                new_level,
            })
        }
        Err(e) => Json(UpgradeBuildingResponse {
            success: false,
            message: format!("Upgrade failed: {}", e),
            new_level: current_level,
        }),
    }
}

/// Unlock a skill for an adventurer.
pub async fn unlock_skill(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UnlockSkillRequest>,
) -> Json<UnlockSkillResponse> {
    let pool = &state.db.pool;

    let adventurer_id = match uuid::Uuid::parse_str(&request.adventurer_id) {
        Ok(id) => id,
        Err(_) => {
            return Json(UnlockSkillResponse {
                success: false,
                message: "Invalid adventurer ID".to_string(),
            });
        }
    };

    // Get adventurer
    let adventurers = queries::get_adventurers(pool, &request.wallet_address)
        .await
        .unwrap_or_default();

    let adventurer = match adventurers.iter().find(|a| a.id == adventurer_id) {
        Some(a) => a,
        None => {
            return Json(UnlockSkillResponse {
                success: false,
                message: "Adventurer not found".to_string(),
            });
        }
    };

    // Check if skill is already unlocked
    if adventurer.skills.contains(&request.skill_id) {
        return Json(UnlockSkillResponse {
            success: false,
            message: "Skill already unlocked".to_string(),
        });
    }

    // Validate skill exists for this class
    let skill_tree = match adventurer.class_key.as_str() {
        "warrior" => shared::SkillTree::warrior(),
        "mage" => shared::SkillTree::mage(),
        "cleric" => shared::SkillTree::cleric(),
        _ => shared::SkillTree::warrior(),
    };

    let skill_node = match skill_tree.nodes.iter().find(|n| n.id == request.skill_id) {
        Some(n) => n,
        None => {
            return Json(UnlockSkillResponse {
                success: false,
                message: "Invalid skill for this class".to_string(),
            });
        }
    };

    // Check tier requirements
    let current_skill_count = adventurer.skills.len() as u32;
    if skill_node.tier > current_skill_count + 1 {
        return Json(UnlockSkillResponse {
            success: false,
            message: format!("Must unlock tier {} skills first", skill_node.tier - 1),
        });
    }

    // Check cost (assume flat 100 gold per skill for now, or use class type data if we had skill costs there)
    // Actually, let's use a flat fee for skills or check class_type if pertinent.
    // For now: 50 Gold * Tier
    let cost = 50 * skill_node.tier;
    if let Err(_) = queries::spend_resources(pool, &request.wallet_address, cost, 0, 0).await {
          return Json(UnlockSkillResponse {
            success: false,
            message: format!("Insufficient gold. Need {} gold.", cost),
        });
    }

    // Add skill to adventurer
    match queries::add_skill(pool, adventurer_id, &request.skill_id).await {
        Ok(_) => {
            println!("Skill unlocked: {} for {}", request.skill_id, adventurer.name);
            Json(UnlockSkillResponse {
                success: true,
                message: format!("Unlocked {}!", skill_node.name),
            })
        }
        Err(e) => Json(UnlockSkillResponse {
            success: false,
            message: format!("Failed to unlock skill: {}", e),
        }),
    }
}

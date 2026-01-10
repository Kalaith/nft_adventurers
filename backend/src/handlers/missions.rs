//! Mission handlers.

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::{missions, queries};
use crate::engine::mission_resolver;
use crate::AppState;
use shared::MissionType;

/// Request to start a mission.
#[derive(Debug, Deserialize)]
pub struct StartMissionRequest {
    pub wallet_address: String,
    pub mission_type: String,
    pub party: Vec<String>, // Adventurer UUIDs as strings
}

/// Response for starting a mission.
#[derive(Debug, Serialize)]
pub struct StartMissionResponse {
    pub success: bool,
    pub mission_id: Option<String>,
    pub message: String,
    pub duration_seconds: Option<u64>,
}

/// Request to resolve a mission.
#[derive(Debug, Deserialize)]
pub struct ResolveMissionRequest {
    pub wallet_address: String,
    pub mission_id: String,
}

/// Response for mission resolution.
#[derive(Debug, Serialize)]
pub struct ResolveMissionResponse {
    pub success: bool,
    pub message: String,
    pub xp_gained: u32,
    pub feats_earned: Vec<String>,
    pub deaths: Vec<String>,
    pub ready: bool,
}

/// Start a new mission.
pub async fn start_mission(
    State(state): State<Arc<AppState>>,
    Json(request): Json<StartMissionRequest>,
) -> Json<StartMissionResponse> {
    let pool = &state.db.pool;

    // Parse mission type
    let mission_type = match request.mission_type.to_lowercase().as_str() {
        "quick_skirmish" => MissionType::QuickSkirmish,
        "dungeon_crawl" => MissionType::DungeonCrawl,
        "boss_raid" => MissionType::BossRaid,
        _ => {
            return Json(StartMissionResponse {
                success: false,
                mission_id: None,
                message: "Invalid mission type".to_string(),
                duration_seconds: None,
            });
        }
    };

    // Parse party UUIDs
    let party: Vec<Uuid> = request
        .party
        .iter()
        .filter_map(|s| Uuid::parse_str(s).ok())
        .collect();

    if party.is_empty() {
        return Json(StartMissionResponse {
            success: false,
            mission_id: None,
            message: "Party cannot be empty".to_string(),
            duration_seconds: None,
        });
    }

    // Verify adventurers are available (healthy status)
    let adventurers = queries::get_adventurers(pool, &request.wallet_address)
        .await
        .unwrap_or_default();

    for adv_id in &party {
        let adv = adventurers.iter().find(|a| a.id == *adv_id);
        match adv {
            Some(a) if !a.is_available() => {
                return Json(StartMissionResponse {
                    success: false,
                    mission_id: None,
                    message: format!("{} is not available", a.name),
                    duration_seconds: None,
                });
            }
            None => {
                return Json(StartMissionResponse {
                    success: false,
                    mission_id: None,
                    message: "Adventurer not found".to_string(),
                    duration_seconds: None,
                });
            }
            _ => {}
        }
    }

    // Check mission cost
    if let Some(mission_data) = queries::get_mission_type_data(pool, &request.mission_type).await.unwrap_or(None) {
        if mission_data.cost_gold > 0 {
            if let Err(_) = queries::spend_resources(pool, &request.wallet_address, mission_data.cost_gold, 0, 0).await {
                return Json(StartMissionResponse {
                    success: false,
                    mission_id: None,
                    message: format!("Insufficient gold. Need {} gold.", mission_data.cost_gold),
                    duration_seconds: None,
                });
            }
        }
    }

    // Create the mission
    // Note: If create_mission fails, we should ideally refund, but for simplicity we assume it works if validations pass.
    // In a production system, this should be in a transaction.
    match missions::create_mission(pool, &request.wallet_address, mission_type, &party).await {
        Ok(mission) => {
            println!(
                "Mission started: {} with {} adventurer(s)",
                mission.id,
                party.len()
            );
            Json(StartMissionResponse {
                success: true,
                mission_id: Some(mission.id.to_string()),
                message: format!("{} started!", mission_type.display_name()),
                duration_seconds: Some(mission.duration_seconds),
            })
        }
        Err(e) => Json(StartMissionResponse {
            success: false,
            mission_id: None,
            message: format!("Failed to start mission: {}", e),
            duration_seconds: None,
        }),
    }
}

/// Resolve a completed mission.
pub async fn resolve_mission(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ResolveMissionRequest>,
) -> Json<ResolveMissionResponse> {
    let pool = &state.db.pool;

    let mission_id = match Uuid::parse_str(&request.mission_id) {
        Ok(id) => id,
        Err(_) => {
            return Json(ResolveMissionResponse {
                success: false,
                message: "Invalid mission ID".to_string(),
                xp_gained: 0,
                feats_earned: Vec::new(),
                deaths: Vec::new(),
                ready: false,
            });
        }
    };

    // Find the mission
    let missions_list = missions::get_active_missions(pool, &request.wallet_address)
        .await
        .unwrap_or_default();

    let mission = match missions_list.into_iter().find(|m| m.id == mission_id) {
        Some(m) => m,
        None => {
            return Json(ResolveMissionResponse {
                success: false,
                message: "Mission not found".to_string(),
                xp_gained: 0,
                feats_earned: Vec::new(),
                deaths: Vec::new(),
                ready: false,
            });
        }
    };

    // Check if mission is complete
    if !mission.is_complete() {
        let remaining = mission.remaining_seconds();
        return Json(ResolveMissionResponse {
            success: false,
            message: format!("Mission not complete. {} seconds remaining.", remaining),
            xp_gained: 0,
            feats_earned: Vec::new(),
            deaths: Vec::new(),
            ready: false,
        });
    }

    // Get adventurers for resolution
    let adventurers = queries::get_adventurers(pool, &request.wallet_address)
        .await
        .unwrap_or_default();

    let party_refs: Vec<&shared::Adventurer> = mission
        .party
        .iter()
        .filter_map(|id| adventurers.iter().find(|a| a.id == *id))
        .collect();

    // Resolve the mission
    let result = mission_resolver::resolve_mission(&mission, &party_refs);

    // Apply results
    let mut feats_earned = Vec::new();
    let mut deaths = Vec::new();

    // Award XP to survivors
    for adv_id in &mission.party {
        if !result.deaths.contains(adv_id) {
            missions::add_xp(pool, *adv_id, result.xp_gained).await.ok();
            missions::heal_adventurer(pool, *adv_id).await.ok();

            // Generate feat for survivor
            if result.success {
                let feat_name = format!("Survived {}", mission.mission_type.display_name());
                missions::append_feat(
                    pool,
                    "adventurer",
                    *adv_id,
                    &feat_name,
                    "survive",
                    "{}",
                )
                .await
                .ok();
                feats_earned.push(feat_name);
            }
        }
    }

    // Award resources if successful
    if result.success {
         if let Some(mission_data) = queries::get_mission_type_data(pool, mission.mission_type.type_key()).await.unwrap_or(None) {
            queries::add_resources(
                pool, 
                &request.wallet_address, 
                mission_data.reward_gold, 
                mission_data.reward_lumber, 
                mission_data.reward_stone
            ).await.ok();
         }
    }

    // Handle deaths
    for dead_id in &result.deaths {
        missions::kill_adventurer(pool, *dead_id).await.ok();
        if let Some(adv) = adventurers.iter().find(|a| a.id == *dead_id) {
            deaths.push(adv.name.clone());

            // Record death feat
            missions::append_feat(
                pool,
                "adventurer",
                *dead_id,
                &format!("Fell in {}", mission.mission_type.display_name()),
                "death",
                "{}",
            )
            .await
            .ok();
        }
    }

    // Delete the completed mission
    missions::delete_mission(pool, mission_id).await.ok();

    let success_msg = if result.success {
        "Mission successful!"
    } else if deaths.is_empty() {
        "Mission failed, but all survived."
    } else {
        "Mission failed with casualties."
    };

    println!(
        "Mission resolved: {} - {} deaths, {} XP",
        mission_id,
        deaths.len(),
        result.xp_gained
    );

    Json(ResolveMissionResponse {
        success: result.success,
        message: success_msg.to_string(),
        xp_gained: result.xp_gained,
        feats_earned,
        deaths,
        ready: true,
    })
}

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::queries;
use crate::AppState;
use shared::Stats;

/// Request to recruit an adventurer.
#[derive(Debug, Deserialize)]
pub struct RecruitRequest {
    pub wallet_address: String,
    pub class_key: String,
    pub name: String,
}

/// Response for recruitment.
#[derive(Debug, Serialize)]
pub struct RecruitResponse {
    pub success: bool,
    pub message: String,
    pub adventurer_id: Option<String>,
}

/// Recruit an adventurer via the Tavern.
pub async fn recruit_adventurer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RecruitRequest>,
) -> Json<RecruitResponse> {
    let pool = &state.db.pool;
    let class_key = request.class_key.to_lowercase();

    // 1. Check if user has a tavern and its level
    let hold = match queries::get_hold(pool, &request.wallet_address).await {
        Ok(h) => h,
        Err(_) => {
            return Json(RecruitResponse {
                success: false,
                message: "Hold not found".to_string(),
                adventurer_id: None,
            });
        }
    };

    let tavern_level = hold.building_level("tavern");

    if tavern_level < 1 {
        return Json(RecruitResponse {
            success: false,
            message: "You must build a Tavern first!".to_string(),
            adventurer_id: None,
        });
    }

    // 2. Validate class availability and get base stats from DB
    let class_data = match queries::get_class_type_data(pool, &class_key).await {
        Ok(Some(data)) => data,
        Ok(None) => {
            return Json(RecruitResponse {
                success: false,
                message: format!("Unknown class: {}", class_key),
                adventurer_id: None,
            });
        }
        Err(_) => {
            return Json(RecruitResponse {
                success: false,
                message: "Database error".to_string(),
                adventurer_id: None,
            });
        }
    };

    if tavern_level < class_data.unlock_level {
         return Json(RecruitResponse {
            success: false,
            message: format!("Tavern Lv.{} required for {}s.", class_data.unlock_level, class_data.display_name),
            adventurer_id: None,
        });
    }

    // 3. Create stats from DB data
    let mut stats = Stats {
        str_: class_data.base_str,
        dex: class_data.base_dex,
        int: class_data.base_int,
        con: class_data.base_con,
        cha: class_data.base_cha,
    };

    // Bonus stats for Lv.4+
    if tavern_level >= 4 {
        stats.str_ += 1;
        stats.dex += 1;
        stats.int += 1;
        stats.con += 1;
        stats.cha += 1;
    }

    // 4. Create Adventurer
    match queries::create_adventurer(pool, &request.wallet_address, &request.name, &class_key, stats).await {
        Ok(adv) => Json(RecruitResponse {
            success: true,
            message: format!("Recruited {} the {}!", request.name, class_key),
            adventurer_id: Some(adv.id.to_string()),
        }),
        Err(e) => Json(RecruitResponse {
            success: false,
            message: format!("Failed to recruit: {}", e),
            adventurer_id: None,
        }),
    }
}

//! Inventory management handlers.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

// ... imports ...
use crate::db::queries;
use crate::AppState;
use shared::EquipSlot;
use sqlx::Row;

/// Request for equipping an item.
#[derive(Debug, Deserialize)]
pub struct EquipRequest {
    pub wallet_address: String,
    pub adventurer_id: String,
    pub item_id: String,
}

/// Request for unequipping an item.
#[derive(Debug, Deserialize)]
pub struct UnequipRequest {
    pub wallet_address: String,
    pub adventurer_id: String,
    pub slot: String,
}

/// Response for inventory operations.
#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub success: bool,
    pub message: String,
}

/// Helper to check if adventurer is owned by wallet and available (not on mission).
async fn check_adventurer_availability(
    pool: &sqlx::SqlitePool,
    adventurer_id: Uuid,
    wallet: &str,
) -> Result<(), StatusCode> {
    let row = sqlx::query("SELECT owner, status FROM adventurers WHERE id = ?")
        .bind(adventurer_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (owner, status) = match row {
        Some(row) => (
            row.get::<String, _>("owner"),
            row.get::<String, _>("status"),
        ),
        None => return Err(StatusCode::NOT_FOUND),
    };

    if owner != wallet {
        return Err(StatusCode::FORBIDDEN);
    }

    if status.starts_with("on_mission") {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Dead adventurers also probably shouldn't mess with gear, but "on_mission" was the specific report.
    // Let's block dead too.
    if status == "dead" {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(())
}

/// Equip an item to an adventurer.
pub async fn equip_item(
    State(state): State<Arc<AppState>>,
    Json(request): Json<EquipRequest>,
) -> Result<Json<InventoryResponse>, StatusCode> {
    let pool = &state.db.pool;

    let adventurer_id = Uuid::parse_str(&request.adventurer_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check availability
    if let Err(code) = check_adventurer_availability(pool, adventurer_id, &request.wallet_address).await {
        return Ok(Json(InventoryResponse {
            success: false,
            message: match code {
                StatusCode::NOT_FOUND => "Adventurer not found".to_string(),
                StatusCode::FORBIDDEN => "Not your adventurer".to_string(),
                StatusCode::BAD_REQUEST => "Adventurer is busy or dead".to_string(),
                _ => "Internal error".to_string(),
            },
        }));
    }

    let item_id = Uuid::parse_str(&request.item_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Verify ownership and get item type
    let items = queries::get_items(pool, &request.wallet_address)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let item = items
        .iter()
        .find(|i| i.id == item_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if item.is_equipped() {
        return Ok(Json(InventoryResponse {
            success: false,
            message: "Item is already equipped".to_string(),
        }));
    }

    let slot = item.equip_slot;

    // Update item's equipped_by field
    match queries::equip_item(pool, item_id, adventurer_id, slot).await {
        Ok(_) => Ok(Json(InventoryResponse {
            success: true,
            message: format!("Equipped {} to adventurer", item.current_name),
        })),
        Err(e) => Ok(Json(InventoryResponse {
            success: false,
            message: format!("Failed to equip: {}", e),
        })),
    }
}

/// Unequip an item from an adventurer.
pub async fn unequip_item(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UnequipRequest>,
) -> Result<Json<InventoryResponse>, StatusCode> {
    let pool = &state.db.pool;

    let adventurer_id = Uuid::parse_str(&request.adventurer_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Check availability
    if let Err(code) = check_adventurer_availability(pool, adventurer_id, &request.wallet_address).await {
         return Ok(Json(InventoryResponse {
            success: false,
            message: match code {
                StatusCode::NOT_FOUND => "Adventurer not found".to_string(),
                StatusCode::FORBIDDEN => "Not your adventurer".to_string(),
                StatusCode::BAD_REQUEST => "Adventurer is busy or dead".to_string(),
                _ => "Internal error".to_string(),
            },
        }));
    }

    let slot = match request.slot.to_lowercase().as_str() {
        "weapon" => EquipSlot::Weapon,
        "armor" => EquipSlot::Armor,
        "accessory" => EquipSlot::Accessory,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    match queries::unequip_slot(pool, adventurer_id, slot).await {
        Ok(_) => Ok(Json(InventoryResponse {
            success: true,
            message: format!("Unequipped {} slot", request.slot),
        })),
        Err(e) => Ok(Json(InventoryResponse {
            success: false,
            message: format!("Failed to unequip: {}", e),
        })),
    }
}

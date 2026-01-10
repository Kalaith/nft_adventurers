//! Player data handlers.

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::{missions, queries};
use crate::AppState;
use shared::{EquipSlot, ItemStats, PlayerData, Rarity, Stats};

/// Request for getting player data.
#[derive(Debug, Deserialize)]
pub struct PlayerRequest {
    pub wallet_address: String,
}

/// Request for minting an adventurer.
#[derive(Debug, Deserialize)]
pub struct MintAdventurerRequest {
    pub wallet_address: String,
    pub name: String,
    pub class: String,
}

/// Request for minting an item.
#[derive(Debug, Deserialize)]
pub struct MintItemRequest {
    pub wallet_address: String,
    pub name: String,
    pub item_type: String,
}

/// Response for mint operations.
#[derive(Debug, Serialize)]
pub struct MintResponse {
    pub success: bool,
    pub id: Option<String>,
    pub message: String,
}

/// Get current player data.
pub async fn get_player_data(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PlayerRequest>,
) -> Result<Json<PlayerData>, StatusCode> {
    let pool = &state.db.pool;
    let wallet = &request.wallet_address;

    // Get or create player
    let player = match queries::get_player(pool, wallet).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            // Create new player with starter content
            queries::create_player(pool, wallet)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Mint starter adventurer with warrior stats
            let warrior_stats = Stats {
                str_: 14, dex: 10, int: 8, con: 12, cha: 8,
            };
            queries::create_adventurer(pool, wallet, "Starter Warrior", "warrior", warrior_stats)
                .await
                .ok();

            // Mint starter sword
            queries::create_item(
                pool,
                wallet,
                "sword",
                EquipSlot::Weapon,
                "Rusty Sword",
                Rarity::Common,
                ItemStats { damage: Some(10), ..Default::default() },
            )
                .await
                .ok();

            queries::get_player(pool, wallet)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Get all player data
    let adventurers = queries::get_adventurers(pool, wallet)
        .await
        .unwrap_or_default();
    let items = queries::get_items(pool, wallet).await.unwrap_or_default();
    let consumables = queries::get_consumables(pool, wallet)
        .await
        .unwrap_or_default();
    let hold = queries::get_hold(pool, wallet)
        .await
        .unwrap_or_else(|_| shared::Hold::new(wallet.to_string()));
    let active_missions = missions::get_active_missions(pool, wallet)
        .await
        .unwrap_or_default();

    Ok(Json(PlayerData {
        player,
        adventurers,
        items,
        consumables,
        hold,
        active_missions,
    }))
}

/// Mint a new adventurer.
pub async fn mint_adventurer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<MintAdventurerRequest>,
) -> Json<MintResponse> {
    let pool = &state.db.pool;

    let class_key = request.class.to_lowercase();
    
    // Default stats based on class_key - in production would look this up from class_types table
    let stats = match class_key.as_str() {
        "warrior" => Stats { str_: 14, dex: 10, int: 8, con: 12, cha: 8 },
        "mage" => Stats { str_: 8, dex: 10, int: 14, con: 10, cha: 10 },
        "cleric" => Stats { str_: 10, dex: 8, int: 10, con: 12, cha: 14 },
        _ => Stats::default(),
    };

    match queries::create_adventurer(pool, &request.wallet_address, &request.name, &class_key, stats).await {
        Ok(adventurer) => Json(MintResponse {
            success: true,
            id: Some(adventurer.id.to_string()),
            message: format!("Minted adventurer: {}", request.name),
        }),
        Err(e) => Json(MintResponse {
            success: false,
            id: None,
            message: format!("Failed to mint: {}", e),
        }),
    }
}

/// Mint a new item.
pub async fn mint_item(
    State(state): State<Arc<AppState>>,
    Json(request): Json<MintItemRequest>,
) -> Json<MintResponse> {
    let pool = &state.db.pool;

    let type_key = request.item_type.to_lowercase();
    
    // Look up equip_slot from type_key
    let equip_slot = match type_key.as_str() {
        "sword" | "staff" | "mace" | "bow" | "dagger" => EquipSlot::Weapon,
        "armor" | "shield" | "helmet" => EquipSlot::Armor,
        "ring" | "amulet" | "cloak" => EquipSlot::Accessory,
        _ => EquipSlot::Weapon,
    };

    match queries::create_item(
        pool,
        &request.wallet_address,
        &type_key,
        equip_slot,
        &request.name,
        Rarity::Common,
        ItemStats::default(),
    )
    .await
    {
        Ok(item) => Json(MintResponse {
            success: true,
            id: Some(item.id.to_string()),
            message: format!("Minted item: {}", request.name),
        }),
        Err(e) => Json(MintResponse {
            success: false,
            id: None,
            message: format!("Failed to mint: {}", e),
        }),
    }
}

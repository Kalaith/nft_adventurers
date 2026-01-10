use axum::{extract::State, Json};
use serde::Deserialize;
use std::sync::Arc;
use sqlx::Acquire; // Added import

use crate::{
    models::ApiResponse,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct BuyItemRequest {
    pub wallet_address: String,
    pub item_type: String,
}

pub async fn buy_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BuyItemRequest>,
) -> Json<ApiResponse<()>> {
    let wallet = &payload.wallet_address;
    let mut conn = match state.db.pool.acquire().await {
        Ok(c) => c,
        Err(e) => return Json(ApiResponse::error(format!("Database error: {}", e))),
    };

    // Begin transaction
    let mut tx = match conn.begin().await {
        Ok(t) => t,
        Err(e) => return Json(ApiResponse::error(format!("Transaction error: {}", e))),
    };

    // 1. Get item cost
    let item_cost: u32 = match sqlx::query_scalar("SELECT cost FROM item_types WHERE type_key = ?")
        .bind(&payload.item_type)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(c)) => c,
        Ok(None) => return Json(ApiResponse::error("Invalid item type")),
        Err(e) => return Json(ApiResponse::error(format!("Database error: {}", e))),
    };

    // 2. Spend resources (Manual query for transaction)
    let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query(
        "UPDATE holds SET gold = gold - ? WHERE player = ? AND gold >= ?"
    )
    .bind(item_cost as i64)
    .bind(wallet)
    .bind(item_cost as i64)
    .execute(&mut *tx)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Json(ApiResponse::error("Insufficient funds"));
            }
        }
        Err(e) => return Json(ApiResponse::error(format!("Database error: {}", e))),
    }

    // 3. Mint item
    let item_id = uuid::Uuid::new_v4().to_string();
    
    // Fetch item details necessary for creation
    let row: Option<(String, String, String)> = match sqlx::query_as(
        r#"
        SELECT display_name, equip_slot, 
        json_object(
            'damage', base_damage,
            'defense', base_defense,
            'healing', base_healing
        ) as stats
        FROM item_types WHERE type_key = ?
        "#
    )
    .bind(&payload.item_type)
    .fetch_optional(&mut *tx)
    .await 
    {
        Ok(row) => row,
        Err(e) => return Json(ApiResponse::error(format!("Failed to fetch item details: {}", e))),
    };
    
    let (base_name, _equip_slot, base_stats_json) = match row {
        Some(r) => r,
        None => return Json(ApiResponse::error("Invalid item type")),
    };

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO items (id, owner, base_type, current_name, rarity, base_stats, created_at)
        VALUES (?, ?, ?, ?, 'common', ?, datetime('now'))
        "#
    )
    .bind(&item_id)
    .bind(wallet)
    .bind(&payload.item_type)
    .bind(&base_name)
    .bind(&base_stats_json)
    .execute(&mut *tx)
    .await
    {
        return Json(ApiResponse::error(format!("Failed to mint item: {}", e)));
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Json(ApiResponse::error(format!("Transaction commit failed: {}", e)));
    }

    Json(ApiResponse::success(()))
}


#[derive(Debug, Deserialize)]
pub struct BuyConsumableRequest {
    pub wallet_address: String,
    pub consumable_type: String,
}

pub async fn buy_consumable(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BuyConsumableRequest>,
) -> Json<ApiResponse<()>> {
    let wallet = &payload.wallet_address;
    let mut conn = match state.db.pool.acquire().await {
        Ok(c) => c,
        Err(e) => return Json(ApiResponse::error(format!("Database error: {}", e))),
    };

    // Begin transaction
    let mut tx = match conn.begin().await {
        Ok(t) => t,
        Err(e) => return Json(ApiResponse::error(format!("Transaction error: {}", e))),
    };

    // 1. Get consumable cost
    let cost: u32 = match sqlx::query_scalar("SELECT cost FROM consumable_types WHERE type_key = ?")
        .bind(&payload.consumable_type)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(c)) => c,
        Ok(None) => return Json(ApiResponse::error("Invalid consumable type")),
        Err(e) => return Json(ApiResponse::error(format!("Database error: {}", e))),
    };

    // 2. Spend gold
    let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query(
        "UPDATE holds SET gold = gold - ? WHERE player = ? AND gold >= ?"
    )
    .bind(cost as i64)
    .bind(wallet)
    .bind(cost as i64)
    .execute(&mut *tx)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Json(ApiResponse::error("Insufficient funds"));
            }
        }
        Err(e) => return Json(ApiResponse::error(format!("Database error: {}", e))),
    }

    // 3. Add consumable
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO consumables (id, owner, consumable_type, quantity)
        VALUES (?, ?, ?, 1)
        ON CONFLICT(owner, consumable_type) DO UPDATE SET quantity = quantity + 1
        "#
    )
    .bind(uuid::Uuid::new_v4().to_string()) 
    .bind(wallet)
    .bind(&payload.consumable_type)
    .execute(&mut *tx)
    .await
    {
        return Json(ApiResponse::error(format!("Failed to add consumable: {}", e)));
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Json(ApiResponse::error(format!("Transaction commit failed: {}", e)));
    }

    Json(ApiResponse::success(()))
}

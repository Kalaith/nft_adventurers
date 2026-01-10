//! Game data handlers (configuration, mission types, etc).

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

use crate::AppState;

/// Mission type data from database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionTypeData {
    pub type_key: String,
    pub display_name: String,
    pub description: String,
    pub duration_seconds: u64,
    pub permadeath_chance: f32,
    pub reward_multiplier: f32,
    pub difficulty_class: u32,
    pub icon_key: String,
}

/// Response containing all mission types.
#[derive(Debug, Serialize)]
pub struct MissionTypesResponse {
    pub mission_types: Vec<MissionTypeData>,
}

/// Get all mission types.
pub async fn get_mission_types(
    State(state): State<Arc<AppState>>,
) -> Json<MissionTypesResponse> {
    let rows = sqlx::query(
        r#"SELECT type_key, display_name, description, duration_seconds, permadeath_chance, reward_multiplier, difficulty_class, icon_key FROM mission_types"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let mission_types: Vec<MissionTypeData> = rows
        .into_iter()
        .map(|row| MissionTypeData {
            type_key: row.get("type_key"),
            display_name: row.get("display_name"),
            description: row.get("description"),
            duration_seconds: row.get::<i64, _>("duration_seconds") as u64,
            permadeath_chance: row.get::<f64, _>("permadeath_chance") as f32,
            reward_multiplier: row.get::<f64, _>("reward_multiplier") as f32,
            difficulty_class: row.get::<i64, _>("difficulty_class") as u32,
            icon_key: row.get("icon_key"),
        })
        .collect();

    Json(MissionTypesResponse { mission_types })
}

/// Item type data from database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeData {
    pub type_key: String,
    pub display_name: String,
    pub icon_key: String,
    pub equip_slot: String,
    pub base_damage: Option<u32>,
    pub base_defense: Option<u32>,
    pub base_healing: Option<u32>,
}

/// Response containing all item types.
#[derive(Debug, Serialize)]
pub struct ItemTypesResponse {
    pub item_types: Vec<ItemTypeData>,
}

/// Get all item types.
pub async fn get_item_types(
    State(state): State<Arc<AppState>>,
) -> Json<ItemTypesResponse> {
    let rows = sqlx::query(
        r#"SELECT type_key, display_name, icon_key, equip_slot, base_damage, base_defense, base_healing FROM item_types"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let item_types: Vec<ItemTypeData> = rows
        .into_iter()
        .map(|row| ItemTypeData {
            type_key: row.get("type_key"),
            display_name: row.get("display_name"),
            icon_key: row.get("icon_key"),
            equip_slot: row.get("equip_slot"),
            base_damage: row.get::<Option<i64>, _>("base_damage").map(|v| v as u32),
            base_defense: row.get::<Option<i64>, _>("base_defense").map(|v| v as u32),
            base_healing: row.get::<Option<i64>, _>("base_healing").map(|v| v as u32),
        })
        .collect();

    Json(ItemTypesResponse { item_types })
}

// ============ Class Types ============

/// Class type data from database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassTypeData {
    pub type_key: String,
    pub display_name: String,
    pub portrait_key: String,
    pub base_str: u32,
    pub base_dex: u32,
    pub base_int: u32,
    pub base_con: u32,
    pub base_cha: u32,
    pub unlock_level: u32,
    pub cost: u32,
}

#[derive(Debug, Serialize)]
pub struct ClassTypesResponse {
    pub class_types: Vec<ClassTypeData>,
}

pub async fn get_class_types(
    State(state): State<Arc<AppState>>,
) -> Json<ClassTypesResponse> {
    let rows = sqlx::query(
        r#"SELECT type_key, display_name, portrait_key, base_str, base_dex, base_int, base_con, base_cha, unlock_level, cost FROM class_types"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let class_types: Vec<ClassTypeData> = rows
        .into_iter()
        .map(|row| ClassTypeData {
            type_key: row.get("type_key"),
            display_name: row.get("display_name"),
            portrait_key: row.get("portrait_key"),
            base_str: row.get::<i64, _>("base_str") as u32,
            base_dex: row.get::<i64, _>("base_dex") as u32,
            base_int: row.get::<i64, _>("base_int") as u32,
            base_con: row.get::<i64, _>("base_con") as u32,
            base_cha: row.get::<i64, _>("base_cha") as u32,
            unlock_level: row.get::<i64, _>("unlock_level") as u32,
            cost: row.get::<i64, _>("cost") as u32,
        })
        .collect();

    Json(ClassTypesResponse { class_types })
}

// ============ Consumable Types ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableTypeData {
    pub type_key: String,
    pub display_name: String,
    pub description: String,
    pub icon_key: String,
    pub effect_type: String,
    pub effect_value: f32,
}

#[derive(Debug, Serialize)]
pub struct ConsumableTypesResponse {
    pub consumable_types: Vec<ConsumableTypeData>,
}

pub async fn get_consumable_types(
    State(state): State<Arc<AppState>>,
) -> Json<ConsumableTypesResponse> {
    let rows = sqlx::query(
        r#"SELECT type_key, display_name, description, icon_key, effect_type, effect_value FROM consumable_types"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let consumable_types: Vec<ConsumableTypeData> = rows
        .into_iter()
        .map(|row| ConsumableTypeData {
            type_key: row.get("type_key"),
            display_name: row.get("display_name"),
            description: row.get("description"),
            icon_key: row.get("icon_key"),
            effect_type: row.get("effect_type"),
            effect_value: row.get::<f64, _>("effect_value") as f32,
        })
        .collect();

    Json(ConsumableTypesResponse { consumable_types })
}

// ============ Building Types ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingTypeData {
    pub type_key: String,
    pub display_name: String,
    pub description: String,
    pub icon_key: String,
    pub xp_bonus_per_level: f32,
}

#[derive(Debug, Serialize)]
pub struct BuildingTypesResponse {
    pub building_types: Vec<BuildingTypeData>,
}

pub async fn get_building_types(
    State(state): State<Arc<AppState>>,
) -> Json<BuildingTypesResponse> {
    let rows = sqlx::query(
        r#"SELECT type_key, display_name, description, icon_key, xp_bonus_per_level FROM building_types"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let building_types: Vec<BuildingTypeData> = rows
        .into_iter()
        .map(|row| BuildingTypeData {
            type_key: row.get("type_key"),
            display_name: row.get("display_name"),
            description: row.get("description"),
            icon_key: row.get("icon_key"),
            xp_bonus_per_level: row.get::<f64, _>("xp_bonus_per_level") as f32,
        })
        .collect();

    Json(BuildingTypesResponse { building_types })
}

//! Database query functions.

use sqlx::SqlitePool;
use uuid::Uuid;

use shared::{Adventurer, AdventurerStatus, Consumable, EquipSlot, Equipment, Hold, Item, ItemStats, Player, Rarity, Stats};
use std::collections::HashMap;

/// Create a new player.
pub async fn create_player(pool: &SqlitePool, wallet: &str) -> Result<Player, sqlx::Error> {
    sqlx::query("INSERT OR IGNORE INTO players (wallet_address) VALUES (?)")
        .bind(wallet)
        .execute(pool)
        .await?;

    // Also create their hold
    sqlx::query("INSERT OR IGNORE INTO holds (player) VALUES (?)")
        .bind(wallet)
        .execute(pool)
        .await?;

    Ok(Player::new(wallet.to_string()))
}

/// Get player by wallet address.
pub async fn get_player(pool: &SqlitePool, wallet: &str) -> Result<Option<Player>, sqlx::Error> {
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT wallet_address, created_at FROM players WHERE wallet_address = ?",
    )
    .bind(wallet)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(wallet, _)| Player::new(wallet)))
}

/// Get all adventurers owned by a wallet.
pub async fn get_adventurers(
    pool: &SqlitePool,
    wallet: &str,
) -> Result<Vec<Adventurer>, sqlx::Error> {
    let rows: Vec<(String, String, String, String, i64, i64, String, String, String)> =
        sqlx::query_as(
            r#"
            SELECT id, owner, name, class, level, xp, stats, skills, status
            FROM adventurers WHERE owner = ?
            "#,
        )
        .bind(wallet)
        .fetch_all(pool)
        .await?;

    let mut adventurers = Vec::new();
    for (id, owner, name, class_key, level, xp, stats_json, skills_json, status) in rows {
        let stats: Stats = serde_json::from_str(&stats_json).unwrap_or_default();
        let skills: Vec<String> = serde_json::from_str(&skills_json).unwrap_or_default();

        let status = if status == "healthy" {
            AdventurerStatus::Healthy
        } else if status == "dead" {
            AdventurerStatus::Dead
        } else if status.starts_with("on_mission:") {
            // Parse "on_mission:UUID" format
            let mission_id_str = status.strip_prefix("on_mission:").unwrap_or("");
            let mission_id = Uuid::parse_str(mission_id_str).unwrap_or_default();
            AdventurerStatus::OnMission { mission_id }
        } else {
            AdventurerStatus::Healthy
        };

        adventurers.push(Adventurer {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            owner,
            name,
            class_key,
            level: level as u32,
            xp: xp as u32,
            stats,
            skills,
            status,
            equipment: Equipment::default(),
        });
    }

    Ok(adventurers)
}

/// Create a new adventurer.
pub async fn create_adventurer(
    pool: &SqlitePool,
    owner: &str,
    name: &str,
    class_key: &str,
    stats: Stats,
) -> Result<Adventurer, sqlx::Error> {
    let adventurer = Adventurer::new(owner.to_string(), name.to_string(), class_key.to_string(), stats);

    let stats_json = serde_json::to_string(&adventurer.stats).unwrap_or_default();
    let skills_json = serde_json::to_string(&adventurer.skills).unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO adventurers (id, owner, name, class, level, xp, stats, skills, status)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'healthy')
        "#,
    )
    .bind(adventurer.id.to_string())
    .bind(owner)
    .bind(name)
    .bind(class_key)
    .bind(adventurer.level as i64)
    .bind(adventurer.xp as i64)
    .bind(&stats_json)
    .bind(&skills_json)
    .execute(pool)
    .await?;

    Ok(adventurer)
}

/// Get all items owned by a wallet.
pub async fn get_items(pool: &SqlitePool, wallet: &str) -> Result<Vec<Item>, sqlx::Error> {
    let rows: Vec<(String, String, String, String, String, String, Option<String>)> =
        sqlx::query_as(
            r#"
            SELECT id, owner, base_type, current_name, rarity, base_stats, equipped_by
            FROM items WHERE owner = ?
            "#,
        )
        .bind(wallet)
        .fetch_all(pool)
        .await?;

    let mut items = Vec::new();
    for (id, owner, type_key, current_name, rarity, base_stats_json, equipped_by) in rows {
        let rarity = match rarity.as_str() {
            "common" => Rarity::Common,
            "uncommon" => Rarity::Uncommon,
            "rare" => Rarity::Rare,
            "epic" => Rarity::Epic,
            "legendary" => Rarity::Legendary,
            "mythic" => Rarity::Mythic,
            _ => Rarity::Common,
        };

        let base_stats: ItemStats = serde_json::from_str(&base_stats_json).unwrap_or_default();
        
        // Look up equip_slot from type_key (simple mapping for now)
        let equip_slot = match type_key.as_str() {
            "sword" | "staff" | "mace" | "bow" | "dagger" => EquipSlot::Weapon,
            "armor" | "shield" | "helmet" => EquipSlot::Armor,
            "ring" | "amulet" | "cloak" => EquipSlot::Accessory,
            _ => EquipSlot::Weapon,
        };

        items.push(Item {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            owner,
            type_key,
            equip_slot,
            current_name,
            rarity,
            base_stats,
            equipped_by: equipped_by.and_then(|s| Uuid::parse_str(&s).ok()),
            feats: Vec::new(),
        });
    }

    Ok(items)
}

/// Create a new item.
pub async fn create_item(
    pool: &SqlitePool,
    owner: &str,
    type_key: &str,
    equip_slot: EquipSlot,
    name: &str,
    rarity: Rarity,
    base_stats: ItemStats,
) -> Result<Item, sqlx::Error> {
    let item = Item::new(
        owner.to_string(),
        type_key.to_string(),
        equip_slot,
        name.to_string(),
        rarity,
        base_stats,
    );

    let rarity_str = match rarity {
        Rarity::Common => "common",
        Rarity::Uncommon => "uncommon",
        Rarity::Rare => "rare",
        Rarity::Epic => "epic",
        Rarity::Legendary => "legendary",
        Rarity::Mythic => "mythic",
    };

    let base_stats_json = serde_json::to_string(&item.base_stats).unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO items (id, owner, base_type, current_name, rarity, base_stats)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(item.id.to_string())
    .bind(owner)
    .bind(type_key)
    .bind(name)
    .bind(rarity_str)
    .bind(&base_stats_json)
    .execute(pool)
    .await?;

    Ok(item)
}

/// Get player's hold.
pub async fn get_hold(pool: &SqlitePool, wallet: &str) -> Result<Hold, sqlx::Error> {
    let row: Option<(String, String, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT buildings, echoes, total_feats, gold, lumber, stone FROM holds WHERE player = ?",
    )
    .bind(wallet)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((buildings_json, echoes_json, total_feats, gold, lumber, stone)) => {
            let buildings: HashMap<String, u32> =
                serde_json::from_str(&buildings_json).unwrap_or_default();
            let echoes = serde_json::from_str(&echoes_json).unwrap_or_default();

            Ok(Hold {
                owner: wallet.to_string(),
                buildings,
                echoes,
                total_feats: total_feats as u32,
                gold: gold as u32,
                lumber: lumber as u32,
                stone: stone as u32,
            })
        }
        None => Ok(Hold::new(wallet.to_string())),
    }
}

/// Spend resources from a player's hold.
pub async fn spend_resources(
    pool: &SqlitePool,
    wallet: &str,
    gold: u32,
    lumber: u32,
    stone: u32,
) -> Result<(), sqlx::Error> {
    // We update only if they have enough. SQLITE check constraints could also work,
    // but returning affected rows is easier for logic control.
    let result = sqlx::query(
        "UPDATE holds SET gold = gold - ?, lumber = lumber - ?, stone = stone - ? WHERE player = ? AND gold >= ? AND lumber >= ? AND stone >= ?"
    )
    .bind(gold as i64)
    .bind(lumber as i64)
    .bind(stone as i64)
    .bind(wallet)
    .bind(gold as i64)
    .bind(lumber as i64)
    .bind(stone as i64)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound); // Treat insufficient funds as "row not found" (conditionally) for now or define custom error
    }

    Ok(())
}

/// Add resources to a player's hold.
pub async fn add_resources(
    pool: &SqlitePool,
    wallet: &str,
    gold: u32,
    lumber: u32,
    stone: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE holds SET gold = gold + ?, lumber = lumber + ?, stone = stone + ? WHERE player = ?"
    )
    .bind(gold as i64)
    .bind(lumber as i64)
    .bind(stone as i64)
    .bind(wallet)
    .execute(pool)
    .await?;

    Ok(())
}

/// Upgrade a building in the player's hold.
pub async fn upgrade_building(
    pool: &SqlitePool,
    wallet: &str,
    building_key: &str,
    new_level: u32,
) -> Result<(), sqlx::Error> {
    // Get current buildings
    let hold = get_hold(pool, wallet).await?;
    let mut buildings = hold.buildings;
    buildings.insert(building_key.to_string(), new_level);

    let buildings_json = serde_json::to_string(&buildings).unwrap_or_default();

    sqlx::query("UPDATE holds SET buildings = ? WHERE player = ?")
        .bind(&buildings_json)
        .bind(wallet)
        .execute(pool)
        .await?;

    Ok(())
}

/// Add a skill to an adventurer.
pub async fn add_skill(
    pool: &SqlitePool,
    adventurer_id: Uuid,
    skill_id: &str,
) -> Result<(), sqlx::Error> {
    // Get current skills
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT skills FROM adventurers WHERE id = ?",
    )
    .bind(adventurer_id.to_string())
    .fetch_optional(pool)
    .await?;

    let mut skills: Vec<String> = match row {
        Some((skills_json,)) => serde_json::from_str(&skills_json).unwrap_or_default(),
        None => Vec::new(),
    };

    skills.push(skill_id.to_string());
    let skills_json = serde_json::to_string(&skills).unwrap_or_default();

    sqlx::query("UPDATE adventurers SET skills = ? WHERE id = ?")
        .bind(&skills_json)
        .bind(adventurer_id.to_string())
        .execute(pool)
        .await?;

    Ok(())
}

/// Get all consumables owned by a wallet.
pub async fn get_consumables(
    pool: &SqlitePool,
    wallet: &str,
) -> Result<Vec<Consumable>, sqlx::Error> {
    let rows: Vec<(String, String, i64)> = sqlx::query_as(
        r#"
        SELECT id, consumable_type, quantity
        FROM consumables WHERE owner = ? AND quantity > 0
        "#,
    )
    .bind(wallet)
    .fetch_all(pool)
    .await?;

    let mut consumables = Vec::new();
    for (id, type_key, quantity) in rows {
        consumables.push(Consumable {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            type_key,
            quantity: quantity as u32,
        });
    }

    Ok(consumables)
}

/// Add consumables to a player's inventory.
pub async fn add_consumable(
    pool: &SqlitePool,
    owner: &str,
    type_key: &str,
    quantity: u32,
) -> Result<(), sqlx::Error> {
    // Upsert: Insert or update quantity if exists
    sqlx::query(
        r#"
        INSERT INTO consumables (id, owner, consumable_type, quantity)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(owner, consumable_type) DO UPDATE SET quantity = quantity + ?
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(owner)
    .bind(type_key)
    .bind(quantity as i64)
    .bind(quantity as i64)
    .execute(pool)
    .await?;

    Ok(())
}

/// Equip an item to an adventurer slot.
pub async fn equip_item(
    pool: &SqlitePool,
    item_id: Uuid,
    adventurer_id: Uuid,
    _slot: shared::EquipSlot,
) -> Result<(), sqlx::Error> {
    // First, unequip any item currently in that slot from any adventurer
    // (The item records which adventurer it's on via equipped_by)

    // Mark the new item as equipped
    sqlx::query("UPDATE items SET equipped_by = ? WHERE id = ?")
        .bind(adventurer_id.to_string())
        .bind(item_id.to_string())
        .execute(pool)
        .await?;

    Ok(())
}

/// Unequip an item from an adventurer slot.
pub async fn unequip_slot(
    pool: &SqlitePool,
    adventurer_id: Uuid,
    slot: shared::EquipSlot,
) -> Result<(), sqlx::Error> {
    // Find items equipped by this adventurer in the given slot
    let slot_types: Vec<&str> = match slot {
        shared::EquipSlot::Weapon => vec!["sword", "staff", "mace"],
        shared::EquipSlot::Armor => vec!["armor", "shield"],
        shared::EquipSlot::Accessory => vec!["ring", "amulet"],
    };

    for item_type in slot_types {
        sqlx::query(
            "UPDATE items SET equipped_by = NULL WHERE equipped_by = ? AND base_type = ?",
        )
        .bind(adventurer_id.to_string())
        .bind(item_type)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Get class type data by key.
pub async fn get_class_type_data(
    pool: &SqlitePool,
    type_key: &str,
) -> Result<Option<shared::ClassTypeData>, sqlx::Error> {
    use sqlx::Row;
    
    let row = sqlx::query(
        "SELECT type_key, display_name, portrait_key, base_str, base_dex, base_int, base_con, base_cha, unlock_level, cost FROM class_types WHERE type_key = ?"
    )
    .bind(type_key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| shared::ClassTypeData {
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
    }))
}

/// Get mission type data by key.
pub async fn get_mission_type_data(
    pool: &SqlitePool,
    type_key: &str,
) -> Result<Option<shared::MissionTypeData>, sqlx::Error> {
    use sqlx::Row;
    
    let row = sqlx::query(
        "SELECT type_key, display_name, description, duration_seconds, permadeath_chance, difficulty_class, cost_gold, reward_gold, reward_lumber, reward_stone, icon_key FROM mission_types WHERE type_key = ?"
    )
    .bind(type_key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| shared::MissionTypeData {
        type_key: row.get("type_key"),
        display_name: row.get("display_name"),
        description: row.get("description"),
        duration_seconds: row.get::<i64, _>("duration_seconds") as u64,
        permadeath_chance: row.get("permadeath_chance"),
        difficulty_class: row.get::<i64, _>("difficulty_class") as u32,
        cost_gold: row.get::<i64, _>("cost_gold") as u32,
        reward_gold: row.get::<i64, _>("reward_gold") as u32,
        reward_lumber: row.get::<i64, _>("reward_lumber") as u32,
        reward_stone: row.get::<i64, _>("reward_stone") as u32,
        icon_key: row.get("icon_key"),
    }))
}

/// Get building type data by key.
pub async fn get_building_type_data(
    pool: &SqlitePool,
    type_key: &str,
) -> Result<Option<shared::BuildingTypeData>, sqlx::Error> {
    use sqlx::Row;
    
    let row = sqlx::query(
        "SELECT type_key, display_name, description, icon_key, xp_bonus_per_level, base_cost_gold, base_cost_lumber, base_cost_stone, cost_scaling FROM building_types WHERE type_key = ?"
    )
    .bind(type_key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| shared::BuildingTypeData {
        type_key: row.get("type_key"),
        display_name: row.get("display_name"),
        description: row.get("description"),
        icon_key: row.get("icon_key"),
        xp_bonus_per_level: row.get("xp_bonus_per_level"),
        base_cost_gold: row.get::<i64, _>("base_cost_gold") as u32,
        base_cost_lumber: row.get::<i64, _>("base_cost_lumber") as u32,
        base_cost_stone: row.get::<i64, _>("base_cost_stone") as u32,
        cost_scaling: row.get("cost_scaling"),
    }))
}



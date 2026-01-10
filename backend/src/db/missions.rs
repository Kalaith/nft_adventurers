//! Mission database queries.

use sqlx::SqlitePool;
use uuid::Uuid;

use shared::{ActiveMission, MissionType};

/// Create a new active mission.
pub async fn create_mission(
    pool: &SqlitePool,
    owner: &str,
    mission_type: MissionType,
    party: &[Uuid],
) -> Result<ActiveMission, sqlx::Error> {
    let mission = ActiveMission::new(owner.to_string(), mission_type, party.to_vec());

    let type_str = match mission_type {
        MissionType::QuickSkirmish => "quick_skirmish",
        MissionType::DungeonCrawl => "dungeon_crawl",
        MissionType::BossRaid => "boss_raid",
    };

    let party_json = serde_json::to_string(&party).unwrap_or_default();
    let consumables_json = serde_json::to_string(&mission.consumables_used).unwrap_or_default();

    sqlx::query(
        r#"
        INSERT INTO active_missions (id, player, mission_type, party, start_time, duration_seconds, consumables_used)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(mission.id.to_string())
    .bind(owner)
    .bind(type_str)
    .bind(&party_json)
    .bind(mission.start_time.to_rfc3339())
    .bind(mission.duration_seconds as i64)
    .bind(&consumables_json)
    .execute(pool)
    .await?;

    // Update adventurer status to on_mission
    for adv_id in party {
        sqlx::query("UPDATE adventurers SET status = ? WHERE id = ?")
            .bind(format!("on_mission:{}", mission.id))
            .bind(adv_id.to_string())
            .execute(pool)
            .await?;
    }

    Ok(mission)
}

/// Get all active missions for a player.
pub async fn get_active_missions(
    pool: &SqlitePool,
    wallet: &str,
) -> Result<Vec<ActiveMission>, sqlx::Error> {
    let rows: Vec<(String, String, String, String, String, i64, String)> = sqlx::query_as(
        r#"
        SELECT id, player, mission_type, party, start_time, duration_seconds, consumables_used
        FROM active_missions WHERE player = ?
        "#,
    )
    .bind(wallet)
    .fetch_all(pool)
    .await?;

    let mut missions = Vec::new();
    for (id, owner, mission_type, party_json, start_time, duration, consumables_json) in rows {
        let mission_type = match mission_type.as_str() {
            "quick_skirmish" => MissionType::QuickSkirmish,
            "dungeon_crawl" => MissionType::DungeonCrawl,
            "boss_raid" => MissionType::BossRaid,
            _ => MissionType::QuickSkirmish,
        };

        let party: Vec<Uuid> = serde_json::from_str(&party_json).unwrap_or_default();
        let consumables: Vec<String> = serde_json::from_str(&consumables_json).unwrap_or_default();

        let start_time = chrono::DateTime::parse_from_rfc3339(&start_time)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

        missions.push(ActiveMission {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            owner,
            mission_type,
            party,
            start_time,
            duration_seconds: duration as u64,
            consumables_used: consumables,
        });
    }

    Ok(missions)
}

/// Delete a completed mission.
pub async fn delete_mission(pool: &SqlitePool, mission_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM active_missions WHERE id = ?")
        .bind(mission_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

/// Mark an adventurer as dead.
pub async fn kill_adventurer(pool: &SqlitePool, adventurer_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE adventurers SET status = 'dead' WHERE id = ?")
        .bind(adventurer_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

/// Mark an adventurer as healthy (after mission).
pub async fn heal_adventurer(pool: &SqlitePool, adventurer_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE adventurers SET status = 'healthy' WHERE id = ?")
        .bind(adventurer_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

/// Add XP to an adventurer.
pub async fn add_xp(pool: &SqlitePool, adventurer_id: Uuid, xp: u32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE adventurers SET xp = xp + ? WHERE id = ?")
        .bind(xp as i64)
        .bind(adventurer_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

/// Append a feat to the ledger.
pub async fn append_feat(
    pool: &SqlitePool,
    entity_type: &str,
    entity_id: Uuid,
    feat_name: &str,
    feat_source: &str,
    bonuses_json: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO feat_ledger (entity_type, entity_id, feat_name, feat_source, bonuses)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(entity_type)
    .bind(entity_id.to_string())
    .bind(feat_name)
    .bind(feat_source)
    .bind(bonuses_json)
    .execute(pool)
    .await?;
    Ok(())
}

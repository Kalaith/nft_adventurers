//! Mission types and active mission tracking.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Mission types available in v1.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionType {
    QuickSkirmish,
    DungeonCrawl,
    BossRaid,
}

impl MissionType {
    /// Get mission duration in seconds.
    pub fn duration_seconds(&self) -> u64 {
        match self {
            MissionType::QuickSkirmish => 4 * 3600,   // 4 hours
            MissionType::DungeonCrawl => 12 * 3600,   // 12 hours
            MissionType::BossRaid => 24 * 3600,       // 24 hours
        }
    }

    /// Get permadeath chance (0.0 - 1.0).
    pub fn permadeath_chance(&self) -> f32 {
        match self {
            MissionType::QuickSkirmish => 0.0,
            MissionType::DungeonCrawl => 0.15,
            MissionType::BossRaid => 0.50,
        }
    }

    /// Get reward multiplier.
    pub fn reward_multiplier(&self) -> f32 {
        match self {
            MissionType::QuickSkirmish => 1.0,
            MissionType::DungeonCrawl => 3.0,
            MissionType::BossRaid => 10.0,
        }
    }

    /// Get difficulty class (DC to beat).
    pub fn difficulty_class(&self) -> u32 {
        match self {
            MissionType::QuickSkirmish => 10,
            MissionType::DungeonCrawl => 15,
            MissionType::BossRaid => 20,
        }
    }

    /// Get display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            MissionType::QuickSkirmish => "Quick Skirmish",
            MissionType::DungeonCrawl => "Dungeon Crawl",
            MissionType::BossRaid => "Boss Raid",
        }
    }

    /// Get description with duration and risk info.
    pub fn description(&self) -> &'static str {
        match self {
            MissionType::QuickSkirmish => "4 hours, safe",
            MissionType::DungeonCrawl => "12 hours, 15% death risk",
            MissionType::BossRaid => "24 hours, 50% death risk",
        }
    }

    /// Get texture key for the mission.
    pub fn icon_key(&self) -> &'static str {
        match self {
            MissionType::QuickSkirmish => "mission_quick_skirmish",
            MissionType::DungeonCrawl => "mission_dungeon_crawl",
            MissionType::BossRaid => "mission_boss_raid",
        }
    }

    /// Get mission type key for API calls.
    pub fn type_key(&self) -> &'static str {
        match self {
            MissionType::QuickSkirmish => "quick_skirmish",
            MissionType::DungeonCrawl => "dungeon_crawl",
            MissionType::BossRaid => "boss_raid",
        }
    }
}

/// An active mission in progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveMission {
    pub id: Uuid,
    pub owner: String,
    pub mission_type: MissionType,
    pub party: Vec<Uuid>,
    pub start_time: DateTime<Utc>,
    pub duration_seconds: u64,
    pub consumables_used: Vec<String>,
}

impl ActiveMission {
    /// Create a new active mission.
    pub fn new(owner: String, mission_type: MissionType, party: Vec<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            owner,
            mission_type,
            party,
            start_time: Utc::now(),
            duration_seconds: mission_type.duration_seconds(),
            consumables_used: Vec::new(),
        }
    }

    /// Check if mission is complete.
    pub fn is_complete(&self) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds() as u64;
        elapsed >= self.duration_seconds
    }

    /// Get progress as 0.0 - 1.0.
    pub fn progress(&self) -> f32 {
        let elapsed = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds() as u64;
        (elapsed as f32 / self.duration_seconds as f32).min(1.0)
    }

    /// Get remaining time in seconds.
    pub fn remaining_seconds(&self) -> u64 {
        let elapsed = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds() as u64;
        self.duration_seconds.saturating_sub(elapsed)
    }
}

/// Result of a completed mission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionResult {
    pub mission_id: Uuid,
    pub success: bool,
    pub xp_gained: u32,
    pub feats_earned: Vec<String>,
    pub deaths: Vec<Uuid>,
    pub loot: Vec<Uuid>,
}

//! Database connection and queries.

pub mod missions;
pub mod queries;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

/// Database connection wrapper.
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    /// Initialize database connection and run migrations.
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // Create the database file if it doesn't exist
        let db_path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
        if !Path::new(db_path).exists() {
            println!("Creating new database: {}", db_path);
            std::fs::File::create(db_path).ok();
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        println!("Database connected: {}", database_url);

        // Run migrations
        let db = Self { pool };
        db.run_migrations().await?;

        Ok(db)
    }

    /// Run database migrations (create tables if not exist).
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        // Players table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS players (
                wallet_address TEXT PRIMARY KEY,
                created_at TEXT DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Adventurers table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS adventurers (
                id TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                level INTEGER DEFAULT 1,
                xp INTEGER DEFAULT 0,
                stats TEXT NOT NULL,
                skills TEXT DEFAULT '[]',
                status TEXT DEFAULT 'healthy',
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (owner) REFERENCES players(wallet_address)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Items table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                base_type TEXT NOT NULL,
                current_name TEXT NOT NULL,
                rarity TEXT NOT NULL,
                base_stats TEXT DEFAULT '{}',
                equipped_by TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (owner) REFERENCES players(wallet_address),
                FOREIGN KEY (equipped_by) REFERENCES adventurers(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Holds table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS holds (
                player TEXT PRIMARY KEY,
                buildings TEXT DEFAULT '{"hearth": 1}',
                echoes TEXT DEFAULT '[]',
                total_feats INTEGER DEFAULT 0,
                gold INTEGER DEFAULT 100,
                lumber INTEGER DEFAULT 50,
                stone INTEGER DEFAULT 0,
                FOREIGN KEY (player) REFERENCES players(wallet_address)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Feat ledger (append-only)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS feat_ledger (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                feat_name TEXT NOT NULL,
                feat_source TEXT NOT NULL,
                bonuses TEXT DEFAULT '{}',
                created_at TEXT DEFAULT (datetime('now'))
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Active missions
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS active_missions (
                id TEXT PRIMARY KEY,
                player TEXT NOT NULL,
                mission_type TEXT NOT NULL,
                party TEXT NOT NULL,
                start_time TEXT NOT NULL,
                duration_seconds INTEGER NOT NULL,
                consumables_used TEXT DEFAULT '[]',
                FOREIGN KEY (player) REFERENCES players(wallet_address)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Consumables table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS consumables (
                id TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                consumable_type TEXT NOT NULL,
                quantity INTEGER DEFAULT 0,
                FOREIGN KEY (owner) REFERENCES players(wallet_address),
                UNIQUE(owner, consumable_type)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Mission types table (game configuration)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS mission_types (
                type_key TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                description TEXT NOT NULL,
                duration_seconds INTEGER NOT NULL,
                permadeath_chance REAL NOT NULL,
                difficulty_class INTEGER NOT NULL,
                cost_gold INTEGER NOT NULL,
                reward_gold INTEGER NOT NULL,
                reward_lumber INTEGER NOT NULL,
                reward_stone INTEGER NOT NULL,
                icon_key TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;



        // Item types table (game configuration)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS item_types (
                type_key TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                icon_key TEXT NOT NULL,
                equip_slot TEXT NOT NULL,
                base_damage INTEGER,
                base_defense INTEGER,
                base_healing INTEGER
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Seed item types
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO item_types (type_key, display_name, icon_key, equip_slot, base_damage, base_defense, base_healing)
            VALUES
                ('sword', 'Sword', 'item_sword', 'weapon', 10, NULL, NULL),
                ('staff', 'Staff', 'item_staff', 'weapon', 8, NULL, NULL),
                ('mace', 'Mace', 'item_mace', 'weapon', 12, NULL, NULL),
                ('armor', 'Armor', 'item_armor', 'armor', NULL, 10, NULL),
                ('shield', 'Shield', 'item_shield', 'armor', NULL, 8, NULL),
                ('ring', 'Ring', 'item_ring', 'accessory', 3, 2, NULL),
                ('amulet', 'Amulet', 'item_amulet', 'accessory', NULL, 5, 5)
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Class types table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS class_types (
                type_key TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                portrait_key TEXT NOT NULL,
                base_str INTEGER NOT NULL,
                base_dex INTEGER NOT NULL,
                base_int INTEGER NOT NULL,
                base_con INTEGER NOT NULL,
                base_cha INTEGER NOT NULL,
                unlock_level INTEGER DEFAULT 1,
                cost INTEGER DEFAULT 0
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Consumable types table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS consumable_types (
                type_key TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                description TEXT NOT NULL,
                icon_key TEXT NOT NULL,
                effect_type TEXT NOT NULL,
                effect_value REAL NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Building types table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS building_types (
                type_key TEXT PRIMARY KEY,
                display_name TEXT NOT NULL,
                description TEXT NOT NULL,
                icon_key TEXT NOT NULL,
                xp_bonus_per_level REAL NOT NULL,
                base_cost_gold INTEGER NOT NULL,
                base_cost_lumber INTEGER NOT NULL,
                base_cost_stone INTEGER NOT NULL,
                cost_scaling REAL NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Verbose Migration Logic
        let migrations = vec![
            ("holds", "gold", "INTEGER DEFAULT 100"),
            ("holds", "lumber", "INTEGER DEFAULT 50"),
            ("holds", "stone", "INTEGER DEFAULT 0"),
            ("mission_types", "cost_gold", "INTEGER DEFAULT 0"),
            ("mission_types", "reward_gold", "INTEGER DEFAULT 0"),
            ("mission_types", "reward_lumber", "INTEGER DEFAULT 0"),
            ("mission_types", "reward_stone", "INTEGER DEFAULT 0"),
            ("building_types", "base_cost_gold", "INTEGER DEFAULT 0"),
            ("building_types", "base_cost_lumber", "INTEGER DEFAULT 0"),
            ("building_types", "base_cost_stone", "INTEGER DEFAULT 0"),
            ("building_types", "cost_scaling", "REAL DEFAULT 1.0"),
            ("item_types", "cost", "INTEGER DEFAULT 50"),
            ("consumable_types", "cost", "INTEGER DEFAULT 20"),
        ];

        for (table, col, def) in migrations {
            let query = format!("ALTER TABLE {} ADD COLUMN {} {}", table, col, def);
            match sqlx::query(&query).execute(&self.pool).await {
                Ok(_) => println!("Migration: Added column {}.{}", table, col),
                Err(e) => {
                    let msg = e.to_string();
                    if !msg.contains("duplicate column") {
                        println!("Migration Note: {}.{} -> {}", table, col, msg);
                    }
                }
            }
        }


        // Seed mission types
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO mission_types (type_key, display_name, description, duration_seconds, permadeath_chance, difficulty_class, cost_gold, reward_gold, reward_lumber, reward_stone, icon_key)
            VALUES
                ('quick_skirmish', 'Quick Skirmish', '4 hours, safe', 14400, 0.0, 10, 0, 10, 5, 0, 'mission_quick_skirmish'),
                ('dungeon_crawl', 'Dungeon Crawl', '12 hours, 15% death risk', 43200, 0.15, 15, 20, 50, 20, 10, 'mission_dungeon_crawl'),
                ('boss_raid', 'Boss Raid', '24 hours, 50% death risk', 86400, 0.50, 20, 100, 500, 100, 50, 'mission_boss_raid')
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Seed item types
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO item_types (type_key, display_name, icon_key, equip_slot, base_damage, base_defense, base_healing, cost)
            VALUES
                ('sword', 'Sword', 'item_sword', 'weapon', 10, NULL, NULL, 50),
                ('staff', 'Staff', 'item_staff', 'weapon', 8, NULL, NULL, 50),
                ('mace', 'Mace', 'item_mace', 'weapon', 12, NULL, NULL, 50),
                ('armor', 'Armor', 'item_armor', 'armor', NULL, 10, NULL, 80),
                ('shield', 'Shield', 'item_shield', 'armor', NULL, 8, NULL, 40),
                ('ring', 'Ring', 'item_ring', 'accessory', 3, 2, NULL, 100),
                ('amulet', 'Amulet', 'item_amulet', 'accessory', NULL, 5, 5, 150)
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Seed class types
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO class_types (type_key, display_name, portrait_key, base_str, base_dex, base_int, base_con, base_cha, unlock_level, cost)
            VALUES
                ('warrior', 'Warrior', 'portrait_warrior', 14, 10, 8, 12, 8, 1, 100),
                ('mage', 'Mage', 'portrait_mage', 8, 10, 14, 10, 10, 1, 100),
                ('cleric', 'Cleric', 'portrait_cleric', 10, 8, 10, 12, 14, 1, 100)
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Seed consumable types
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO consumable_types (type_key, display_name, description, icon_key, effect_type, effect_value, cost)
            VALUES
                ('health_potion', 'Health Potion', '+50% HP restore mid-mission', 'item_health_potion', 'hp_restore', 0.5, 25),
                ('fire_resistance', 'Fire Resistance', '80% fire damage resistance', 'item_fire_resistance', 'fire_resist', 0.8, 50),
                ('peril_veil', 'Peril Veil', '-15% permadeath chance', 'item_peril_veil', 'permadeath_reduce', 0.15, 100)
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Seed building types
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO building_types (type_key, display_name, description, icon_key, xp_bonus_per_level, base_cost_gold, base_cost_lumber, base_cost_stone, cost_scaling)
            VALUES
                ('hearth', 'Hearth', 'Central gathering place', 'building_hearth', 0.05, 0, 0, 0, 0.0),
                ('training_yard', 'Training Yard', 'Train adventurers', 'building_training', 0.03, 50, 20, 10, 1.5),
                ('feat_anvil', 'Feat Anvil', 'Forge feat bonuses', 'building_anvil', 0.08, 100, 50, 20, 1.5),
                ('tavern', 'Tavern', 'Recruit new adventurers', 'building_tavern', 0.0, 200, 100, 50, 1.5),
                ('smithy', 'Smithy', 'Buy equipment', 'building_smithy', 0.0, 150, 50, 20, 1.5),
                ('market', 'Market', 'Buy consumables', 'building_market', 0.0, 100, 30, 0, 1.5)
            "#,
        )
        .execute(&self.pool)
        .await?;
        
        // Force update mission types/building types data with new rewards/costs (Update existing rows to defaults/balanced values)
        // This query runs every startup but updates are cheap.
        let _ = sqlx::query("UPDATE mission_types SET cost_gold = 0, reward_gold = 10, reward_lumber = 5, reward_stone = 0 WHERE type_key = 'quick_skirmish'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE mission_types SET cost_gold = 20, reward_gold = 50, reward_lumber = 20, reward_stone = 10 WHERE type_key = 'dungeon_crawl'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE mission_types SET cost_gold = 100, reward_gold = 500, reward_lumber = 100, reward_stone = 50 WHERE type_key = 'boss_raid'").execute(&self.pool).await;
        
        let _ = sqlx::query("UPDATE building_types SET base_cost_gold = 50, base_cost_lumber = 20, base_cost_stone = 10, cost_scaling = 1.5 WHERE type_key = 'training_yard'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE building_types SET base_cost_gold = 100, base_cost_lumber = 50, base_cost_stone = 20, cost_scaling = 1.5 WHERE type_key = 'feat_anvil'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE building_types SET base_cost_gold = 200, base_cost_lumber = 100, base_cost_stone = 50, cost_scaling = 1.5 WHERE type_key = 'tavern'").execute(&self.pool).await;

        let _ = sqlx::query("UPDATE class_types SET cost = 100 WHERE type_key = 'warrior'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE class_types SET cost = 100 WHERE type_key = 'mage'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE class_types SET cost = 100 WHERE type_key = 'cleric'").execute(&self.pool).await;

        // Ensure new buildings have correct costs if they existed but were wrong (though they are new)
        let _ = sqlx::query("UPDATE building_types SET base_cost_gold = 150, base_cost_lumber = 50, base_cost_stone = 20, cost_scaling = 1.5 WHERE type_key = 'smithy'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE building_types SET base_cost_gold = 100, base_cost_lumber = 30, base_cost_stone = 0, cost_scaling = 1.5 WHERE type_key = 'market'").execute(&self.pool).await;

        // Initialize item/consumable costs
        let _ = sqlx::query("UPDATE item_types SET cost = 50 WHERE type_key IN ('sword', 'staff', 'mace')").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE item_types SET cost = 80 WHERE type_key = 'armor'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE item_types SET cost = 40 WHERE type_key = 'shield'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE item_types SET cost = 100 WHERE type_key = 'ring'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE item_types SET cost = 150 WHERE type_key = 'amulet'").execute(&self.pool).await;

        let _ = sqlx::query("UPDATE consumable_types SET cost = 25 WHERE type_key = 'health_potion'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE consumable_types SET cost = 50 WHERE type_key = 'fire_resistance'").execute(&self.pool).await;
        let _ = sqlx::query("UPDATE consumable_types SET cost = 100 WHERE type_key = 'peril_veil'").execute(&self.pool).await;

        println!("Database migrations complete");
        Ok(())
    }
}

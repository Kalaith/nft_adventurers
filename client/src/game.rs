//! Game loop and state machine.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;

use crate::api::ApiClient;
use crate::state::{GameState, StateTransition};
use shared::PlayerData;

/// Connection status for UI display.
#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected { wallet: String },
    Error(String),
}

/// Main game struct managing state and resources.
pub struct Game {
    pub state: GameState,
    pub api: ApiClient,
    pub connection_status: ConnectionStatus,
    pub last_challenge: Option<String>,
    pub pending_action: Option<PendingAction>,
    pub player_data: Option<PlayerData>,
    pub selected_adventurer: Option<String>,
    pub status_message: Option<String>,
    pub assets: AssetManager,
    pub class_types: Vec<shared::ClassTypeData>,
}

/// Actions that need to be processed.
#[derive(Debug, Clone)]
pub enum PendingAction {
    Connect,
    Disconnect,
    RefreshPlayerData,
    SelectAdventurer(String),
    StartMission { mission_type: String, adventurer_id: String },
    ResolveMission(String),
    UpgradeBuilding(String),
    UnlockSkill { adventurer_id: String, skill_id: String },
    RecruitAdventurer { class_key: String, name: String },
    GoToMissions,
    GoToHold,
    GoToHoldUpgrades,
    GoToSkills(String),
    GoToRecruit,
    GoToInventory,
    GoToAdventurerDetail(String),
    EquipItem { adventurer_id: String, item_id: String },
    UnequipSlot { adventurer_id: String, slot: String },
}

impl Game {
    /// Create a new game instance.
    pub async fn new() -> Self {
        let mut assets = AssetManager::new();
        
        // Load all game textures
        let texture_list = [
            // Portraits
            ("portrait_warrior", "client/assets/portrait_warrior.png"),
            ("portrait_mage", "client/assets/portrait_mage.png"),
            ("portrait_cleric", "client/assets/portrait_cleric.png"),
            // Buildings
            ("building_hearth", "client/assets/building_hearth.png"),
            ("building_training_yard", "client/assets/building_training_yard.png"),
            ("building_feat_anvil", "client/assets/building_feat_anvil.png"),
            // Enemies
            ("enemy_goblin", "client/assets/enemy_goblin.png"),
            ("enemy_rat", "client/assets/enemy_rat.png"),
            ("enemy_skeleton", "client/assets/enemy_skeleton.png"),
            ("enemy_orc", "client/assets/enemy_orc.png"),
            ("enemy_goblin_chief", "client/assets/enemy_goblin_chief.png"),
            ("enemy_dragon", "client/assets/enemy_dragon.png"),
            ("enemy_demon", "client/assets/enemy_demon.png"),
            ("enemy_lich", "client/assets/enemy_lich.png"),
            // Missions
            ("mission_quick_skirmish", "client/assets/mission_quick_skirmish.png"),
            ("mission_dungeon_crawl", "client/assets/mission_dungeon_crawl.png"),
            ("mission_boss_raid", "client/assets/mission_boss_raid.png"),
            // Items
            ("item_sword", "client/assets/item_sword.png"),
            ("item_staff", "client/assets/item_staff.png"),
            ("item_shield", "client/assets/item_shield.png"),
            ("item_armor", "client/assets/item_armor.png"),
            ("item_ring", "client/assets/item_ring.png"),
            ("item_amulet", "client/assets/item_amulet.png"),
            ("item_holy_symbol", "client/assets/item_holy_symbol.png"),
            // UI Backgrounds
            ("ui_background_main", "client/assets/ui_background_main.png"),
            ("ui_background_mission_select", "client/assets/ui_background_mission_select.png"),
        ];
        
        for (name, path) in texture_list {
            if let Err(e) = assets.load_texture(name, path).await {
                eprintln!("Warning: {}", e);
            }
        }
        
        println!("Loaded {} textures", assets.len());
        
        Self {
            state: GameState::MainMenu,
            api: ApiClient::new(),
            connection_status: ConnectionStatus::Disconnected,
            last_challenge: None,
            pending_action: None,
            player_data: None,
            selected_adventurer: None,
            status_message: None,
            assets,
            class_types: Vec::new(),
        }
    }

    /// Update game state each frame.
    pub fn update(&mut self) {
        if let Some(action) = self.pending_action.take() {
            match action {
                PendingAction::Connect => {
                    self.connection_status = ConnectionStatus::Connecting;
                    self.state = GameState::Connecting;
                    return;
                }
                PendingAction::Disconnect => {
                    self.connection_status = ConnectionStatus::Disconnected;
                    self.player_data = None;
                    self.state = GameState::MainMenu;
                    return;
                }
                PendingAction::RefreshPlayerData => {
                    self.load_player_data();
                    return;
                }
                PendingAction::SelectAdventurer(id) => {
                    self.selected_adventurer = Some(id);
                    return;
                }
                PendingAction::StartMission { mission_type, adventurer_id } => {
                    self.start_mission(&mission_type, &adventurer_id);
                    return;
                }
                PendingAction::ResolveMission(mission_id) => {
                    self.resolve_mission(&mission_id);
                    return;
                }
                PendingAction::UpgradeBuilding(building) => {
                    self.upgrade_building(&building);
                    return;
                }
                PendingAction::UnlockSkill { adventurer_id, skill_id } => {
                    self.unlock_skill(&adventurer_id, &skill_id);
                    return;
                }
                PendingAction::GoToMissions => {
                    if let Some(data) = &self.player_data {
                        self.selected_adventurer = data
                            .adventurers
                            .iter()
                            .find(|a| a.is_available())
                            .map(|a| a.id.to_string());
                    }
                    self.state = GameState::MissionSelect;
                    return;
                }
                PendingAction::GoToHold => {
                    self.state = GameState::Hold;
                    return;
                }
                PendingAction::GoToHoldUpgrades => {
                    self.state = GameState::HoldUpgrades;
                    return;
                }
                PendingAction::GoToSkills(adv_id) => {
                    self.state = GameState::Skills { adventurer_id: adv_id };
                    return;
                }
                PendingAction::GoToInventory => {
                    self.state = GameState::Inventory;
                    return;
                }
                PendingAction::GoToAdventurerDetail(adv_id) => {
                    self.state = GameState::AdventurerDetail { adventurer_id: adv_id };
                    return;
                }
                PendingAction::EquipItem { adventurer_id, item_id } => {
                    self.equip_item(&adventurer_id, &item_id);
                    return;
                }
                PendingAction::UnequipSlot { adventurer_id, slot } => {
                    self.unequip_slot(&adventurer_id, &slot);
                    return;
                }
                PendingAction::GoToRecruit => {
                    self.state = GameState::Recruit;
                    // Ensure we have class data
                    if self.class_types.is_empty() {
                         self.load_game_data();
                    }
                    return;
                }
                PendingAction::RecruitAdventurer { class_key, name } => {
                    self.recruit_adventurer(&class_key, &name);
                    return;
                }
            }
        }

        let transition = match &self.state {
            GameState::MainMenu => None,
            GameState::Connecting => self.update_connecting(),
            _ => None,
        };

        if let Some(t) = transition {
            self.apply_transition(t);
        }
    }

    fn load_player_data(&mut self) {
        match self.api.get_player_data() {
            Ok(data) => {
                self.player_data = Some(data);
            }
            Err(e) => {
                eprintln!("Failed to load player data: {}", e);
            }
        }
    }

    fn start_mission(&mut self, mission_type: &str, adventurer_id: &str) {
        let party = vec![adventurer_id.to_string()];

        match self.api.start_mission(mission_type, party) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                    self.selected_adventurer = None;
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn resolve_mission(&mut self, mission_id: &str) {
        match self.api.resolve_mission(mission_id) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn upgrade_building(&mut self, building: &str) {
        match self.api.upgrade_building(building) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn unlock_skill(&mut self, adventurer_id: &str, skill_id: &str) {
        match self.api.unlock_skill(adventurer_id, skill_id) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn equip_item(&mut self, adventurer_id: &str, item_id: &str) {
        match self.api.equip_item(adventurer_id, item_id) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn unequip_slot(&mut self, adventurer_id: &str, slot: &str) {
        match self.api.unequip_slot(adventurer_id, slot) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn load_game_data(&mut self) {
        match self.api.get_class_types() {
             Ok(classes) => {
                 self.class_types = classes;
             }
             Err(e) => {
                 eprintln!("Failed to load class types: {}", e);
             }
        }
    }

    fn recruit_adventurer(&mut self, class_key: &str, name: &str) {
        match self.api.recruit_adventurer(class_key, name) {
            Ok(response) => {
                self.status_message = Some(response.message);
                if response.success {
                    self.load_player_data();
                    self.state = GameState::Hold; // Return to hold after successful recruitment
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    pub fn draw(&mut self) {
        clear_background(dark::BACKGROUND);

        // Draw current screen and collect any action
        let screen_action = match self.state.clone() {
            GameState::MainMenu => crate::ui::screens::main_menu::draw_main_menu(),
            GameState::Connecting => {
                crate::ui::screens::main_menu::draw_connecting();
                None
            }
            GameState::Hold => {
                crate::ui::screens::hold::draw(self.player_data.as_ref(), &self.assets)
            }
            GameState::HoldUpgrades => {
                crate::ui::screens::hold::draw_upgrades(self.player_data.as_ref(), &self.assets)
            }
            GameState::Skills { adventurer_id } => {
                crate::ui::screens::skills::draw(&adventurer_id, self.player_data.as_ref())
            }
            GameState::MissionSelect => {
                crate::ui::screens::mission_select::draw(
                    self.player_data.as_ref(),
                    &self.selected_adventurer,
                    &self.assets,
                )
            }
            GameState::Inventory => {
                crate::ui::screens::inventory::draw(self.player_data.as_ref(), &self.assets)
            }
            GameState::AdventurerDetail { adventurer_id } => {
                crate::ui::screens::adventurer_detail::draw(
                    &adventurer_id,
                    self.player_data.as_ref(),
                    &self.assets,
                )
            }
            GameState::Recruit => {
                crate::ui::screens::recruit::draw(
                    self.player_data.as_ref(),
                    &self.class_types,
                    &self.assets,
                )
            }
        };
        
        // Apply any action from the screen
        if let Some(action) = screen_action {
            self.pending_action = Some(action);
        }

        self.draw_connection_status();
        self.draw_status_message();
    }

    fn apply_transition(&mut self, transition: StateTransition) {
        self.state = match transition {
            StateTransition::ToMainMenu => GameState::MainMenu,
            StateTransition::ToHold => GameState::Hold,
        };
    }

    fn update_connecting(&mut self) -> Option<StateTransition> {
        match self.api.get_challenge() {
            Ok(challenge) => {
                self.last_challenge = Some(challenge.nonce.clone());
                let mock_wallet = "0xDev1234567890abcdef";
                let mock_signature = "mock_sig";

                match self.api.verify(mock_wallet, mock_signature, &challenge.nonce) {
                    Ok(session) => {
                        if session.success {
                            self.connection_status = ConnectionStatus::Connected {
                                wallet: session
                                    .wallet_address
                                    .unwrap_or_else(|| mock_wallet.to_string()),
                            };
                            self.load_player_data();
                            return Some(StateTransition::ToHold);
                        } else {
                            self.connection_status =
                                ConnectionStatus::Error("Auth failed".to_string());
                        }
                    }
                    Err(e) => {
                        self.connection_status = ConnectionStatus::Error(e);
                    }
                }
            }
            Err(e) => {
                self.connection_status = ConnectionStatus::Error(e);
            }
        }
        Some(StateTransition::ToMainMenu)
    }

    fn draw_main_menu(&mut self) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let title = "NFT Adventurers";
        let title_size = 48.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            center_x - title_dims.width / 2.0,
            center_y - 100.0,
            title_size,
            dark::TEXT_BRIGHT,
        );

        let subtitle = "Legends Forged in Chain";
        let sub_size = 24.0;
        let sub_dims = measure_text(subtitle, None, sub_size as u16, 1.0);
        draw_text(
            subtitle,
            center_x - sub_dims.width / 2.0,
            center_y - 60.0,
            sub_size,
            dark::TEXT_DIM,
        );

        if macroquad_toolkit::ui::button(
            center_x - 100.0,
            center_y,
            200.0,
            50.0,
            "Connect Wallet",
        ) {
            self.pending_action = Some(PendingAction::Connect);
        }
    }

    fn draw_connecting(&self) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        draw_text("Connecting...", center_x - 60.0, center_y, 24.0, dark::TEXT);
    }

    fn draw_hold(&mut self) {
        // Draw background
        if let Some(tex) = self.assets.get_texture("ui_background_main") {
            let scale_x = screen_width() / tex.width();
            let scale_y = screen_height() / tex.height();
            let scale = scale_x.max(scale_y);
            draw_texture_ex(
                tex,
                0.0,
                0.0,
                Color::new(1.0, 1.0, 1.0, 0.3), // Dim the background
                DrawTextureParams {
                    dest_size: Some(vec2(tex.width() * scale, tex.height() * scale)),
                    ..Default::default()
                },
            );
        }
        
        // Semi-transparent overlay for readability
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.05, 0.05, 0.1, 0.7));

        let padding = 20.0;
        let panel_width = screen_width() - padding * 2.0;
        
        // Title bar
        draw_rectangle(padding, padding, panel_width, 50.0, Color::new(0.15, 0.12, 0.2, 0.9));
        draw_rectangle_lines(padding, padding, panel_width, 50.0, 2.0, Color::new(0.6, 0.5, 0.8, 0.5));
        draw_text("Your Hold", padding + 15.0, padding + 35.0, 32.0, Color::new(0.9, 0.85, 1.0, 1.0));

        let mut y = padding + 70.0;

        if let Some(data) = &self.player_data {
            // Adventurers panel
            let adv_panel_height = 30.0 + (data.adventurers.len() as f32 * 80.0);
            draw_rectangle(padding, y, panel_width * 0.55, adv_panel_height, Color::new(0.1, 0.1, 0.15, 0.85));
            draw_rectangle_lines(padding, y, panel_width * 0.55, adv_panel_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
            
            draw_text("⚔ Adventurers", padding + 15.0, y + 22.0, 20.0, Color::new(0.7, 0.8, 1.0, 1.0));
            
            let mut card_y = y + 35.0;
            let adventurers: Vec<_> = data.adventurers.iter().collect();
            for adv in &adventurers {
                // Adventurer card
                let card_x = padding + 10.0;
                let card_width = panel_width * 0.55 - 20.0;
                let card_height = 70.0;
                
                // Card background with status-based color
                let card_color = match &adv.status {
                    shared::AdventurerStatus::Healthy => Color::new(0.08, 0.15, 0.1, 0.9),
                    shared::AdventurerStatus::OnMission { .. } => Color::new(0.15, 0.12, 0.08, 0.9),
                    shared::AdventurerStatus::Injured { .. } => Color::new(0.18, 0.08, 0.08, 0.9),
                    shared::AdventurerStatus::Dead => Color::new(0.1, 0.1, 0.1, 0.9),
                };
                draw_rectangle(card_x, card_y, card_width, card_height, card_color);
                draw_rectangle_lines(card_x, card_y, card_width, card_height, 1.0, Color::new(0.5, 0.5, 0.5, 0.4));
                
                // Portrait
                let portrait_key = format!("portrait_{}", adv.class_key);
                if let Some(tex) = self.assets.get_texture(&portrait_key) {
                    let portrait_size = 60.0;
                    let scale = portrait_size / tex.height();
                    draw_texture_ex(
                        tex,
                        card_x + 5.0,
                        card_y + 5.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(tex.width() * scale, portrait_size)),
                            ..Default::default()
                        },
                    );
                }
                
                // Adventurer info
                let text_x = card_x + 75.0;
                let class_str = {
                    let mut s = adv.class_key.clone();
                    if let Some(c) = s.get_mut(0..1) {
                        c.make_ascii_uppercase();
                    }
                    s
                };
                
                draw_text(&adv.name, text_x, card_y + 22.0, 20.0, WHITE);
                draw_text(
                    &format!("{} • Level {}", class_str, adv.level),
                    text_x,
                    card_y + 42.0,
                    14.0,
                    Color::new(0.7, 0.7, 0.7, 1.0),
                );
                
                // Status badge
                let (status_str, status_color) = match &adv.status {
                    shared::AdventurerStatus::Healthy => ("Ready", Color::new(0.3, 0.8, 0.3, 1.0)),
                    shared::AdventurerStatus::OnMission { .. } => ("On Mission", Color::new(0.9, 0.7, 0.2, 1.0)),
                    shared::AdventurerStatus::Injured { .. } => ("Injured", Color::new(0.9, 0.4, 0.3, 1.0)),
                    shared::AdventurerStatus::Dead => ("Dead", Color::new(0.5, 0.5, 0.5, 1.0)),
                };
                draw_text(status_str, text_x, card_y + 60.0, 14.0, status_color);
                
                // Skills button
                if macroquad_toolkit::ui::button(card_x + card_width - 90.0, card_y + 25.0, 80.0, 28.0, "Skills") {
                    self.pending_action = Some(PendingAction::GoToSkills(adv.id.to_string()));
                }
                
                card_y += card_height + 8.0;
            }

            // Missions panel (right side)
            let missions_x = padding + panel_width * 0.57;
            let missions_width = panel_width * 0.43;
            let missions_height = 30.0 + (data.active_missions.len() as f32 * 50.0).max(60.0);
            
            draw_rectangle(missions_x, y, missions_width, missions_height, Color::new(0.1, 0.1, 0.15, 0.85));
            draw_rectangle_lines(missions_x, y, missions_width, missions_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
            
            draw_text("⏱ Active Missions", missions_x + 15.0, y + 22.0, 20.0, Color::new(0.9, 0.8, 0.5, 1.0));
            
            let mut mission_y = y + 40.0;
            if data.active_missions.is_empty() {
                draw_text("No active missions", missions_x + 15.0, mission_y + 15.0, 14.0, Color::new(0.5, 0.5, 0.5, 1.0));
            } else {
                for mission in &data.active_missions {
                    let remaining = mission.remaining_seconds();
                    let is_complete = mission.is_complete();
                    let time_str = if is_complete {
                        "COMPLETE!".to_string()
                    } else {
                        format!("{}:{:02}", remaining / 60, remaining % 60)
                    };
                    
                    let time_color = if is_complete { 
                        Color::new(0.3, 1.0, 0.3, 1.0) 
                    } else { 
                        Color::new(0.8, 0.8, 0.6, 1.0) 
                    };
                    
                    draw_text(
                        &format!("• {}", mission.mission_type.display_name()),
                        missions_x + 15.0,
                        mission_y,
                        16.0,
                        WHITE,
                    );
                    draw_text(&time_str, missions_x + missions_width - 100.0, mission_y, 14.0, time_color);
                    
                    if is_complete {
                        if macroquad_toolkit::ui::button(missions_x + missions_width - 85.0, mission_y + 5.0, 70.0, 24.0, "Claim") {
                            self.pending_action = Some(PendingAction::ResolveMission(mission.id.to_string()));
                        }
                        mission_y += 35.0;
                    }
                    mission_y += 20.0;
                }
            }
        }

        // Bottom navigation bar
        let nav_y = screen_height() - 70.0;
        draw_rectangle(0.0, nav_y, screen_width(), 70.0, Color::new(0.08, 0.08, 0.12, 0.95));
        draw_line(0.0, nav_y, screen_width(), nav_y, 2.0, Color::new(0.4, 0.3, 0.6, 0.5));
        
        let btn_y = nav_y + 15.0;
        if macroquad_toolkit::ui::button(padding, btn_y, 100.0, 40.0, "⚔ Missions") {
            self.pending_action = Some(PendingAction::GoToMissions);
        }
        if macroquad_toolkit::ui::button(padding + 110.0, btn_y, 100.0, 40.0, "🏰 Upgrades") {
            self.pending_action = Some(PendingAction::GoToHoldUpgrades);
        }
        if macroquad_toolkit::ui::button(padding + 220.0, btn_y, 100.0, 40.0, "📦 Inventory") {
            self.pending_action = Some(PendingAction::GoToInventory);
        }
        if macroquad_toolkit::ui::button(padding + 330.0, btn_y, 80.0, 40.0, "↻ Refresh") {
            self.pending_action = Some(PendingAction::RefreshPlayerData);
        }
        if macroquad_toolkit::ui::button(screen_width() - padding - 80.0, btn_y, 80.0, 40.0, "Logout") {
            self.pending_action = Some(PendingAction::Disconnect);
        }
    }


    fn draw_hold_upgrades(&mut self) {
        let mut y = 40.0;

        draw_text("Building Upgrades", 20.0, y, 32.0, dark::TEXT_BRIGHT);
        y += 50.0;

        let buildings = [
            ("hearth", "building_hearth", "Hearth", "+5% XP per level"),
            ("training_yard", "building_training_yard", "Training Yard", "Faster adventurer recovery"),
            ("feat_anvil", "building_feat_anvil", "Feat Anvil", "+8% XP per level"),
        ];

        if let Some(data) = &self.player_data {
            for (id, tex_key, name, desc) in buildings {
                // Draw building image
                if let Some(tex) = self.assets.get_texture(tex_key) {
                    let scale = 64.0 / tex.height();
                    draw_texture_ex(
                        tex,
                        30.0,
                        y - 10.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(tex.width() * scale, 64.0)),
                            ..Default::default()
                        },
                    );
                }
                
                let level = data.hold.building_level(id);

                draw_text(
                    &format!("{} (Lv.{})", name, level),
                    110.0,
                    y + 20.0,
                    22.0,
                    dark::TEXT_BRIGHT,
                );
                draw_text(desc, 110.0, y + 42.0, 14.0, dark::TEXT_DIM);

                if level < 5 {
                    if macroquad_toolkit::ui::button(110.0, y + 55.0, 120.0, 30.0, "Upgrade") {
                        self.pending_action = Some(PendingAction::UpgradeBuilding(id.to_string()));
                    }
                } else {
                    draw_text("MAX LEVEL", 110.0, y + 60.0, 14.0, dark::POSITIVE);
                }
                y += 100.0;
            }
        }

        if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
            self.pending_action = Some(PendingAction::GoToHold);
        }
    }

    fn draw_skills(&mut self, adventurer_id: &str) {
        let mut y = 40.0;

        // Find adventurer
        let adventurer = self
            .player_data
            .as_ref()
            .and_then(|data| {
                data.adventurers
                    .iter()
                    .find(|a| a.id.to_string() == adventurer_id)
            })
            .cloned();

        let adv = match adventurer {
            Some(a) => a,
            None => {
                draw_text("Adventurer not found", 20.0, y, 24.0, dark::NEGATIVE);
                if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
                    self.pending_action = Some(PendingAction::GoToHold);
                }
                return;
            }
        };

        draw_text(
            &format!("{} - Skills", adv.name),
            20.0,
            y,
            32.0,
            dark::TEXT_BRIGHT,
        );
        y += 50.0;

        // Get skill tree for this class
        let skill_tree = match adv.class_key.as_str() {
            "warrior" => shared::SkillTree::warrior(),
            "mage" => shared::SkillTree::mage(),
            "cleric" => shared::SkillTree::cleric(),
            _ => shared::SkillTree::warrior(),
        };

        let current_tier = adv.skills.len() as u32;

        for node in &skill_tree.nodes {
            let unlocked = adv.skills.contains(&node.id);
            let can_unlock = !unlocked && node.tier == current_tier + 1;

            let color = if unlocked {
                dark::POSITIVE
            } else if can_unlock {
                dark::ACCENT
            } else {
                dark::TEXT_DIM
            };

            draw_text(
                &format!("[Tier {}] {}", node.tier, node.name),
                30.0,
                y,
                20.0,
                color,
            );
            y += 20.0;
            draw_text(&node.description, 30.0, y, 14.0, dark::TEXT_DIM);
            y += 20.0;

            if can_unlock {
                if macroquad_toolkit::ui::button(30.0, y, 100.0, 25.0, "Unlock") {
                    self.pending_action = Some(PendingAction::UnlockSkill {
                        adventurer_id: adventurer_id.to_string(),
                        skill_id: node.id.clone(),
                    });
                }
                y += 30.0;
            } else if unlocked {
                draw_text("✓ Unlocked", 30.0, y, 14.0, dark::POSITIVE);
                y += 20.0;
            }
            y += 15.0;
        }

        if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
            self.pending_action = Some(PendingAction::GoToHold);
        }
    }

    fn draw_mission_select(&mut self) {
        let mut y = 40.0;

        draw_text("Select Mission", 20.0, y, 32.0, dark::TEXT_BRIGHT);
        y += 40.0;

        // Adventurer selection
        draw_text("Adventurer:", 20.0, y, 20.0, dark::ACCENT);
        y += 25.0;

        let mut available = Vec::new();
        if let Some(data) = &self.player_data {
            for adv in &data.adventurers {
                if adv.is_available() {
                    available.push((adv.id.to_string(), adv.name.clone(), adv.level));
                }
            }
        }

        if available.is_empty() {
            draw_text("No adventurers available!", 30.0, y, 18.0, dark::NEGATIVE);
            y += 30.0;
        } else {
            for (id, name, level) in &available {
                let selected = self.selected_adventurer.as_ref() == Some(id);
                let label = if selected {
                    format!("✓ {} Lv.{}", name, level)
                } else {
                    format!("  {} Lv.{}", name, level)
                };

                if macroquad_toolkit::ui::button(30.0, y, 200.0, 28.0, &label) {
                    self.pending_action = Some(PendingAction::SelectAdventurer(id.clone()));
                }
                y += 32.0;
            }
        }

        y += 20.0;
        draw_text("Mission:", 20.0, y, 20.0, dark::ACCENT);
        y += 30.0;

        let missions = [
            ("quick_skirmish", "mission_quick_skirmish", "Quick Skirmish", "5s, safe"),
            ("dungeon_crawl", "mission_dungeon_crawl", "Dungeon Crawl", "30s, risky"),
            ("boss_raid", "mission_boss_raid", "Boss Raid", "60s, deadly"),
        ];

        let can_start = self.selected_adventurer.is_some();

        for (id, tex_key, name, desc) in missions {
            // Draw mission thumbnail
            if let Some(tex) = self.assets.get_texture(tex_key) {
                let thumb_height = 60.0;
                let scale = thumb_height / tex.height();
                draw_texture_ex(
                    tex,
                    30.0,
                    y - 5.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(tex.width() * scale, thumb_height)),
                        ..Default::default()
                    },
                );
            }
            
            draw_text(name, 150.0, y + 20.0, 18.0, dark::TEXT_BRIGHT);
            draw_text(desc, 150.0, y + 40.0, 14.0, dark::TEXT_DIM);

            if can_start {
                if macroquad_toolkit::ui::button(350.0, y + 15.0, 80.0, 28.0, "Go") {
                    if let Some(adv_id) = &self.selected_adventurer {
                        self.pending_action = Some(PendingAction::StartMission {
                            mission_type: id.to_string(),
                            adventurer_id: adv_id.clone(),
                        });
                    }
                }
            }
            y += 70.0;
        }


        if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
            self.pending_action = Some(PendingAction::GoToHold);
        }
    }

    fn draw_inventory(&mut self) {
        // Draw background overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.05, 0.05, 0.1, 0.95));

        let padding = 20.0;
        let panel_width = screen_width() - padding * 2.0;
        
        // Title bar
        draw_rectangle(padding, padding, panel_width, 50.0, Color::new(0.15, 0.12, 0.2, 0.9));
        draw_rectangle_lines(padding, padding, panel_width, 50.0, 2.0, Color::new(0.6, 0.5, 0.8, 0.5));
        draw_text("📦 Inventory", padding + 15.0, padding + 35.0, 32.0, Color::new(0.9, 0.85, 1.0, 1.0));

        let mut y = padding + 70.0;

        if let Some(data) = &self.player_data {
            // Items section
            let item_panel_height = 200.0;
            draw_rectangle(padding, y, panel_width, item_panel_height, Color::new(0.1, 0.1, 0.15, 0.85));
            draw_rectangle_lines(padding, y, panel_width, item_panel_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
            
            draw_text("⚔ Equipment", padding + 15.0, y + 22.0, 20.0, Color::new(0.7, 0.8, 1.0, 1.0));
            
            if data.items.is_empty() {
                draw_text("No items yet. Complete missions to earn loot!", padding + 15.0, y + 60.0, 16.0, Color::new(0.5, 0.5, 0.5, 1.0));
            } else {
                let mut item_x = padding + 15.0;
                let mut item_y = y + 40.0;
                let item_size = 64.0;
                
                for item in &data.items {
                    // Item card with rarity color
                    let rarity_color = match item.rarity {
                        shared::Rarity::Common => Color::new(0.3, 0.3, 0.3, 0.8),
                        shared::Rarity::Uncommon => Color::new(0.2, 0.4, 0.2, 0.8),
                        shared::Rarity::Rare => Color::new(0.2, 0.3, 0.5, 0.8),
                        shared::Rarity::Epic => Color::new(0.4, 0.2, 0.5, 0.8),
                        shared::Rarity::Legendary => Color::new(0.5, 0.4, 0.1, 0.8),
                        shared::Rarity::Mythic => Color::new(0.5, 0.2, 0.2, 0.8),
                    };
                    draw_rectangle(item_x, item_y, item_size, item_size + 20.0, rarity_color);
                    draw_rectangle_lines(item_x, item_y, item_size, item_size + 20.0, 1.0, Color::new(0.6, 0.6, 0.6, 0.6));
                    
                    // Item icon
                    if let Some(tex) = self.assets.get_texture(&format!("item_{}", item.type_key)) {
                        let scale = (item_size - 8.0) / tex.height().max(tex.width());
                        draw_texture_ex(
                            tex,
                            item_x + 4.0,
                            item_y + 4.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(tex.width() * scale, tex.height() * scale)),
                                ..Default::default()
                            },
                        );
                    }
                    
                    // Item name
                    let display_name = if item.current_name.len() > 8 {
                        format!("{}...", &item.current_name[..6])
                    } else {
                        item.current_name.clone()
                    };
                    draw_text(&display_name, item_x + 2.0, item_y + item_size + 14.0, 10.0, WHITE);
                    
                    if item.is_equipped() {
                        draw_text("E", item_x + item_size - 12.0, item_y + 12.0, 12.0, Color::new(0.3, 1.0, 0.3, 1.0));
                    }
                    
                    item_x += item_size + 10.0;
                    if item_x + item_size > padding + panel_width - 15.0 {
                        item_x = padding + 15.0;
                        item_y += item_size + 30.0;
                    }
                }
            }
            
            y += item_panel_height + 15.0;

            // Consumables section
            let cons_panel_height = 120.0;
            draw_rectangle(padding, y, panel_width, cons_panel_height, Color::new(0.1, 0.1, 0.15, 0.85));
            draw_rectangle_lines(padding, y, panel_width, cons_panel_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
            
            draw_text("🧪 Consumables", padding + 15.0, y + 22.0, 20.0, Color::new(0.8, 0.7, 1.0, 1.0));
            
            if data.consumables.is_empty() {
                draw_text("No consumables. Find potions on missions!", padding + 15.0, y + 60.0, 16.0, Color::new(0.5, 0.5, 0.5, 1.0));
            } else {
                let mut cons_x = padding + 15.0;
                for consumable in &data.consumables {
                    let card_width = 150.0;
                    draw_rectangle(cons_x, y + 35.0, card_width, 70.0, Color::new(0.15, 0.12, 0.18, 0.9));
                    draw_rectangle_lines(cons_x, y + 35.0, card_width, 70.0, 1.0, Color::new(0.5, 0.4, 0.6, 0.5));
                    
                    // Display name: capitalize type_key and replace underscores
                    let display_name = {
                        let s = consumable.type_key.replace('_', " ");
                        let mut chars = s.chars().collect::<Vec<_>>();
                        if let Some(c) = chars.get_mut(0) {
                            *c = c.to_ascii_uppercase();
                        }
                        chars.into_iter().collect::<String>()
                    };
                    draw_text(&display_name, cons_x + 10.0, y + 55.0, 14.0, WHITE);
                    draw_text(&format!("Type: {}", consumable.type_key), cons_x + 10.0, y + 72.0, 10.0, Color::new(0.6, 0.6, 0.6, 1.0));
                    draw_text(&format!("x{}", consumable.quantity), cons_x + card_width - 30.0, y + 55.0, 16.0, Color::new(0.9, 0.8, 0.3, 1.0));
                    
                    cons_x += card_width + 10.0;
                }
            }
        }

        // Back button
        if macroquad_toolkit::ui::button(padding, screen_height() - 60.0, 100.0, 40.0, "← Back") {
            self.pending_action = Some(PendingAction::GoToHold);
        }
    }

    fn draw_connection_status(&self) {
        let status_text = match &self.connection_status {
            ConnectionStatus::Disconnected => "Not connected".to_string(),
            ConnectionStatus::Connecting => "Connecting...".to_string(),
            ConnectionStatus::Connected { wallet } => {
                format!("{}...", &wallet[..10.min(wallet.len())])
            }
            ConnectionStatus::Error(e) => format!("Error: {}", e),
        };

        let color = match &self.connection_status {
            ConnectionStatus::Disconnected => dark::TEXT_DIM,
            ConnectionStatus::Connecting => dark::WARNING,
            ConnectionStatus::Connected { .. } => dark::POSITIVE,
            ConnectionStatus::Error(_) => dark::NEGATIVE,
        };

        draw_text(&status_text, 10.0, 20.0, 14.0, color);
    }

    fn draw_status_message(&mut self) {
        if let Some(msg) = &self.status_message {
            draw_text(msg, 10.0, screen_height() - 20.0, 16.0, dark::WARNING);
        }
    }
}

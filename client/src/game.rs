//! Game loop and state machine.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;

use crate::api::ApiClient;
use crate::state::{GameState, StateTransition};
use macroquad_toolkit::ui::draw_ui_text;
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
    pub building_types: Vec<shared::BuildingTypeData>,
    pub item_types: Vec<shared::ItemTypeData>,
    pub consumable_types: Vec<shared::ConsumableTypeData>,
    pub identity: Option<crate::identity::Identity>,
}

/// Actions that that need to be processed.
#[derive(Debug, Clone)]
pub enum PendingAction {
    Connect,
    Disconnect,
    #[allow(dead_code)]
    RefreshPlayerData,
    SelectAdventurer(String),
    StartMission {
        mission_type: String,
        adventurer_id: String,
    },
    ResolveMission(String),
    UpgradeBuilding(String),
    UnlockSkill {
        adventurer_id: String,
        skill_id: String,
    },
    RecruitAdventurer {
        class_key: String,
        name: String,
    },
    GoToMissions,
    GoToHold,
    GoToHoldUpgrades,
    GoToSkills(String),
    GoToRecruit,
    GoToInventory,
    GoToAdventurerDetail(String),
    EquipItem {
        adventurer_id: String,
        item_id: String,
    },
    UnequipSlot {
        adventurer_id: String,
        slot: String,
    },
    GoToSmithy,
    GoToMarket,
    BuyItem(String),
    BuyConsumable(String),
}

impl Game {
    /// Create a new game instance.
    pub async fn new() -> Self {
        let mut assets = AssetManager::new();

        // Load textures using external configuration
        let textures = crate::data::assets::TextureConfig::load_textures();

        for entry in textures {
            match assets.load_texture(&entry.key, &entry.path).await {
                Ok(_) => println!("Loaded: {}", entry.key),
                Err(e) => eprintln!(
                    "Warning: Failed to load {} from '{}': {}",
                    entry.key, entry.path, e
                ),
            }
        }

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
            building_types: Vec::new(),
            item_types: Vec::new(),
            consumable_types: Vec::new(),
            identity: Some(crate::identity::Identity::new_dev()),
        }
    }

    /// Update game state each frame.
    pub async fn update(&mut self) {
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
                PendingAction::StartMission {
                    mission_type,
                    adventurer_id,
                } => {
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
                PendingAction::UnlockSkill {
                    adventurer_id,
                    skill_id,
                } => {
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
                    self.state = GameState::HoldUpgrades { scroll: 0.0 };
                    return;
                }
                PendingAction::GoToSkills(adv_id) => {
                    self.state = GameState::Skills {
                        adventurer_id: adv_id,
                    };
                    return;
                }
                PendingAction::GoToInventory => {
                    self.state = GameState::Inventory;
                    return;
                }
                PendingAction::GoToAdventurerDetail(adv_id) => {
                    self.state = GameState::AdventurerDetail {
                        adventurer_id: adv_id,
                    };
                    return;
                }
                PendingAction::EquipItem {
                    adventurer_id,
                    item_id,
                } => {
                    self.equip_item(&adventurer_id, &item_id);
                    return;
                }
                PendingAction::UnequipSlot {
                    adventurer_id,
                    slot,
                } => {
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
                PendingAction::GoToSmithy => {
                    self.state = GameState::Smithy;
                    return;
                }
                PendingAction::GoToMarket => {
                    self.state = GameState::Market;
                    return;
                }
                PendingAction::BuyItem(item_type) => {
                    self.buy_item(&item_type);
                    return;
                }
                PendingAction::BuyConsumable(consumable_type) => {
                    self.buy_consumable(&consumable_type);
                    return;
                }
            }
        }

        let transition = match &self.state {
            GameState::MainMenu => None,
            GameState::Connecting => self.update_connecting().await,
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
                self.status_message = Some(format!(
                    "{} New level: {}",
                    response.message, response.new_level
                ));
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

        match self.api.get_building_types() {
            Ok(buildings) => {
                self.building_types = buildings;
            }
            Err(e) => {
                eprintln!("Failed to load building types: {}", e);
            }
        }

        match self.api.get_item_types() {
            Ok(items) => {
                self.item_types = items;
            }
            Err(e) => {
                eprintln!("Failed to load item types: {}", e);
            }
        }

        match self.api.get_consumable_types() {
            Ok(consumables) => {
                self.consumable_types = consumables;
            }
            Err(e) => {
                eprintln!("Failed to load consumable types: {}", e);
            }
        }
    }

    fn recruit_adventurer(&mut self, class_key: &str, name: &str) {
        match self.api.recruit_adventurer(class_key, name) {
            Ok(response) => {
                self.status_message = Some(match response.adventurer_id {
                    Some(id) => format!("{} Adventurer: {}", response.message, id),
                    None => response.message,
                });
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

    fn buy_item(&mut self, item_type: &str) {
        match self.api.buy_item(item_type) {
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

    fn buy_consumable(&mut self, consumable_type: &str) {
        match self.api.buy_consumable(consumable_type) {
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
            GameState::HoldUpgrades { ref mut scroll } => crate::ui::screens::hold::draw_upgrades(
                self.player_data.as_ref(),
                &self.building_types,
                &self.assets,
                scroll,
            ),
            GameState::Skills { adventurer_id } => {
                crate::ui::screens::skills::draw(&adventurer_id, self.player_data.as_ref())
            }
            GameState::MissionSelect => crate::ui::screens::mission_select::draw(
                self.player_data.as_ref(),
                &self.selected_adventurer,
                &self.assets,
            ),
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
            GameState::Recruit => crate::ui::screens::recruit::draw(
                self.player_data.as_ref(),
                &self.class_types,
                &self.assets,
            ),
            GameState::Smithy => crate::ui::screens::market::draw_smithy(
                self.player_data.as_ref(),
                &self.item_types,
                &self.assets,
            ),
            GameState::Market => crate::ui::screens::market::draw_market(
                self.player_data.as_ref(),
                &self.consumable_types,
                &self.assets,
            ),
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

    async fn update_connecting(&mut self) -> Option<StateTransition> {
        match self.api.get_challenge() {
            Ok(challenge) => {
                self.last_challenge = Some(challenge.nonce.clone());
                let identity = self.identity.as_ref().expect("No identity found");
                let wallet_address = identity.address();
                let signature = identity.sign(&challenge.message).await;

                match self
                    .api
                    .verify(&wallet_address, &signature, &challenge.nonce)
                {
                    Ok(session) => {
                        if session.success {
                            self.connection_status = ConnectionStatus::Connected {
                                wallet: session
                                    .wallet_address
                                    .unwrap_or_else(|| wallet_address.to_string()),
                            };
                            self.load_player_data();
                            self.load_game_data();
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

        draw_ui_text(&status_text, 10.0, 20.0, 14.0, color);
    }

    fn draw_status_message(&mut self) {
        if let Some(msg) = &self.status_message {
            draw_ui_text(msg, 10.0, screen_height() - 20.0, 16.0, dark::WARNING);
        }
    }
}

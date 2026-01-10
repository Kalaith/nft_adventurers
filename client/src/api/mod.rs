//! API client for backend communication.

use serde::{Deserialize, Serialize};
use shared::PlayerData;

/// Backend API base URL.
const API_BASE: &str = "http://127.0.0.1:3000";

/// Auth challenge response from backend.
#[derive(Debug, Clone, Deserialize)]
pub struct ChallengeResponse {
    pub nonce: String,
    pub message: String,
}

/// Auth verify request.
#[derive(Debug, Serialize)]
pub struct VerifyRequest {
    pub wallet_address: String,
    pub signature: String,
    pub nonce: String,
}

/// Session response after verification.
#[derive(Debug, Clone, Deserialize)]
pub struct SessionResponse {
    pub success: bool,
    pub wallet_address: Option<String>,
    pub session_token: Option<String>,
}

/// Player request.
#[derive(Debug, Serialize)]
pub struct PlayerRequest {
    pub wallet_address: String,
}

/// Start mission request.
#[derive(Debug, Serialize)]
pub struct StartMissionRequest {
    pub wallet_address: String,
    pub mission_type: String,
    pub party: Vec<String>,
}

/// Start mission response.
#[derive(Debug, Clone, Deserialize)]
pub struct StartMissionResponse {
    pub success: bool,
    pub message: String,
}

/// Resolve mission request.
#[derive(Debug, Serialize)]
pub struct ResolveMissionRequest {
    pub wallet_address: String,
    pub mission_id: String,
}

/// Resolve mission response.
#[derive(Debug, Clone, Deserialize)]
pub struct ResolveMissionResponse {
    pub success: bool,
    pub message: String,
}

/// Upgrade building request.
#[derive(Debug, Serialize)]
pub struct UpgradeBuildingRequest {
    pub wallet_address: String,
    pub building: String,
}

/// Upgrade building response.
#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeBuildingResponse {
    pub success: bool,
    pub message: String,
    pub new_level: u32,
}

/// Unlock skill request.
#[derive(Debug, Serialize)]
pub struct UnlockSkillRequest {
    pub wallet_address: String,
    pub adventurer_id: String,
    pub skill_id: String,
}

/// Unlock skill response.
#[derive(Debug, Clone, Deserialize)]
pub struct UnlockSkillResponse {
    pub success: bool,
    pub message: String,
}

/// Equip item request.
#[derive(Debug, Serialize)]
pub struct EquipItemRequest {
    pub wallet_address: String,
    pub adventurer_id: String,
    pub item_id: String,
}

/// Unequip slot request.
#[derive(Debug, Serialize)]
pub struct UnequipSlotRequest {
    pub wallet_address: String,
    pub adventurer_id: String,
    pub slot: String,
}

/// Inventory operation response.
#[derive(Debug, Clone, Deserialize)]
pub struct InventoryResponse {
    pub success: bool,
    pub message: String,
}

/// Mission types response.
#[derive(Debug, Clone, Deserialize)]
pub struct MissionTypesResponse {
    pub mission_types: Vec<shared::MissionTypeData>,
}

/// Item types response.
#[derive(Debug, Clone, Deserialize)]
pub struct ItemTypesResponse {
    pub item_types: Vec<shared::ItemTypeData>,
}

/// Recruit adventurer request.
#[derive(Debug, Serialize)]
pub struct RecruitRequest {
    pub wallet_address: String,
    pub class_key: String,
    pub name: String,
}

/// Recruit adventurer response.
#[derive(Debug, Clone, Deserialize)]
pub struct RecruitResponse {
    pub success: bool,
    pub message: String,
    pub adventurer_id: Option<String>,
}

/// API client for communicating with backend.
pub struct ApiClient {
    base_url: String,
    session_token: Option<String>,
    pub wallet_address: Option<String>,
}

impl ApiClient {
    /// Create a new API client.
    pub fn new() -> Self {
        Self {
            base_url: API_BASE.to_string(),
            session_token: None,
            wallet_address: None,
        }
    }

    /// Request auth challenge from backend.
    pub fn get_challenge(&self) -> Result<ChallengeResponse, String> {
        let url = format!("{}/api/auth/challenge", self.base_url);
        match ureq::get(&url).call() {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Verify wallet signature.
    pub fn verify(
        &mut self,
        wallet: &str,
        signature: &str,
        nonce: &str,
    ) -> Result<SessionResponse, String> {
        let url = format!("{}/api/auth/verify", self.base_url);
        let request = VerifyRequest {
            wallet_address: wallet.to_string(),
            signature: signature.to_string(),
            nonce: nonce.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => {
                let session: SessionResponse = response
                    .into_json()
                    .map_err(|e| format!("Parse error: {}", e))?;

                if session.success {
                    self.session_token = session.session_token.clone();
                    self.wallet_address = session.wallet_address.clone();
                }
                Ok(session)
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Get player data from backend.
    pub fn get_player_data(&self) -> Result<PlayerData, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/player/me", self.base_url);
        let request = PlayerRequest {
            wallet_address: wallet.clone(),
        };

        match ureq::get(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Start a mission.
    pub fn start_mission(
        &self,
        mission_type: &str,
        party: Vec<String>,
    ) -> Result<StartMissionResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/mission/start", self.base_url);
        let request = StartMissionRequest {
            wallet_address: wallet.clone(),
            mission_type: mission_type.to_string(),
            party,
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Resolve a completed mission.
    pub fn resolve_mission(&self, mission_id: &str) -> Result<ResolveMissionResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/mission/resolve", self.base_url);
        let request = ResolveMissionRequest {
            wallet_address: wallet.clone(),
            mission_id: mission_id.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Upgrade a building in the hold.
    pub fn upgrade_building(&self, building: &str) -> Result<UpgradeBuildingResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/hold/upgrade", self.base_url);
        let request = UpgradeBuildingRequest {
            wallet_address: wallet.clone(),
            building: building.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Unlock a skill for an adventurer.
    pub fn unlock_skill(
        &self,
        adventurer_id: &str,
        skill_id: &str,
    ) -> Result<UnlockSkillResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/skill/unlock", self.base_url);
        let request = UnlockSkillRequest {
            wallet_address: wallet.clone(),
            adventurer_id: adventurer_id.to_string(),
            skill_id: skill_id.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Equip an item to an adventurer.
    pub fn equip_item(
        &self,
        adventurer_id: &str,
        item_id: &str,
    ) -> Result<InventoryResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/inventory/equip", self.base_url);
        let request = EquipItemRequest {
            wallet_address: wallet.clone(),
            adventurer_id: adventurer_id.to_string(),
            item_id: item_id.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Unequip an item from an adventurer slot.
    pub fn unequip_slot(
        &self,
        adventurer_id: &str,
        slot: &str,
    ) -> Result<InventoryResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/inventory/unequip", self.base_url);
        let request = UnequipSlotRequest {
            wallet_address: wallet.clone(),
            adventurer_id: adventurer_id.to_string(),
            slot: slot.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Get mission types from backend.
    pub fn get_mission_types(&self) -> Result<Vec<shared::MissionTypeData>, String> {
        let url = format!("{}/api/game/mission-types", self.base_url);
        match ureq::get(&url).call() {
            Ok(response) => {
                let resp: MissionTypesResponse = response
                    .into_json()
                    .map_err(|e| format!("Parse error: {}", e))?;
                Ok(resp.mission_types)
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Get item types from backend.
    pub fn get_item_types(&self) -> Result<Vec<shared::ItemTypeData>, String> {
        let url = format!("{}/api/game/item-types", self.base_url);
        match ureq::get(&url).call() {
            Ok(response) => {
                let resp: ItemTypesResponse = response
                    .into_json()
                    .map_err(|e| format!("Parse error: {}", e))?;
                Ok(resp.item_types)
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Recruit a new adventurer.
    pub fn recruit_adventurer(
        &self,
        class_key: &str,
        name: &str,
    ) -> Result<RecruitResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;

        let url = format!("{}/api/tavern/recruit", self.base_url);
        let request = RecruitRequest {
            wallet_address: wallet.clone(),
            class_key: class_key.to_string(),
            name: name.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Get class types from backend.
    pub fn get_class_types(&self) -> Result<Vec<shared::ClassTypeData>, String> {
        let url = format!("{}/api/game/class-types", self.base_url);
        match ureq::get(&url).call() {
            Ok(response) => {
                let resp: ClassTypesResponse = response
                    .into_json()
                    .map_err(|e| format!("Parse error: {}", e))?;
                Ok(resp.class_types)
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

/// Class types response.
#[derive(Debug, Clone, Deserialize)]
pub struct ClassTypesResponse {
    pub class_types: Vec<shared::ClassTypeData>,
}

/// Building types response.
#[derive(Debug, Clone, Deserialize)]
pub struct BuildingTypesResponse {
    pub building_types: Vec<shared::BuildingTypeData>,
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiClient {
    /// Get building types from backend.
    pub fn get_building_types(&self) -> Result<Vec<shared::BuildingTypeData>, String> {
        let url = format!("{}/api/game/building-types", self.base_url);
        match ureq::get(&url).call() {
            Ok(response) => {
                let resp: BuildingTypesResponse = response
                    .into_json()
                    .map_err(|e| format!("Parse error: {}", e))?;
                Ok(resp.building_types)
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Buy an item from the Smithy.
    pub fn buy_item(&self, item_type: &str) -> Result<MarketResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;
        let url = format!("{}/api/market/buy-item", self.base_url);
        
        let request = BuyItemRequest {
            wallet_address: wallet.clone(),
            item_type: item_type.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Buy a consumable from the Market.
    pub fn buy_consumable(&self, consumable_type: &str) -> Result<MarketResponse, String> {
        let wallet = self.wallet_address.as_ref().ok_or("Not authenticated")?;
        let url = format!("{}/api/market/buy-consumable", self.base_url);
        
        let request = BuyConsumableRequest {
            wallet_address: wallet.clone(),
            consumable_type: consumable_type.to_string(),
        };

        match ureq::post(&url).send_json(&request) {
            Ok(response) => response
                .into_json()
                .map_err(|e| format!("Parse error: {}", e)),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    /// Get consumable types from backend.
    pub fn get_consumable_types(&self) -> Result<Vec<shared::ConsumableTypeData>, String> {
        let url = format!("{}/api/game/consumable-types", self.base_url);
        match ureq::get(&url).call() {
            Ok(response) => {
                let resp: ConsumableTypesResponse = response
                    .into_json()
                    .map_err(|e| format!("Parse error: {}", e))?;
                Ok(resp.consumable_types)
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

/// Buy item request.
#[derive(Debug, Serialize)]
pub struct BuyItemRequest {
    pub wallet_address: String,
    pub item_type: String,
}

/// Buy consumable request.
#[derive(Debug, Serialize)]
pub struct BuyConsumableRequest {
    pub wallet_address: String,
    pub consumable_type: String,
}

/// Generic market action response.
#[derive(Debug, Clone, Deserialize)]
pub struct MarketResponse {
    pub success: bool,
    pub message: String,
}

/// Consumable types response.
#[derive(Debug, Clone, Deserialize)]
pub struct ConsumableTypesResponse {
    pub consumable_types: Vec<shared::ConsumableTypeData>,
}

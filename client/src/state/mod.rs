//! Game state machine.

/// All possible game states.
#[derive(Debug, Clone)]
pub enum GameState {
    MainMenu,
    Connecting,
    Hold,
    HoldUpgrades,
    Skills { adventurer_id: String },
    MissionSelect,
    Inventory,
    AdventurerDetail { adventurer_id: String },
    Recruit,
}

/// State transitions.
#[derive(Debug, Clone)]
pub enum StateTransition {
    ToMainMenu,
    ToHold,
}

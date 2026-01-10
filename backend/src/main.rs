//! NFT Adventurers - Backend Server
//!
//! REST API server for the NFT Adventurers game.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod db;
mod engine;
mod handlers;
mod models;

use db::Database;

/// Shared application state.
pub struct AppState {
    pub db: Database,
}

#[tokio::main]
async fn main() {
    println!("NFT Adventurers Backend starting...");

    // Initialize database
    let db = Database::new("sqlite:nft_adventurers.db")
        .await
        .expect("Failed to connect to database");

    let state = Arc::new(AppState { db });

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api_routes())
        .with_state(state)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint.
async fn health_check() -> &'static str {
    "OK"
}

/// API routes.
fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/challenge", get(handlers::auth::challenge))
        .route("/auth/verify", post(handlers::auth::verify))
        .route("/player/me", get(handlers::player::get_player_data))
        .route("/adventurer/mint", post(handlers::player::mint_adventurer))
        .route("/item/mint", post(handlers::player::mint_item))
        .route("/tavern/recruit", post(handlers::tavern::recruit_adventurer))
        .route("/mission/start", post(handlers::missions::start_mission))
        .route("/mission/resolve", post(handlers::missions::resolve_mission))
        .route("/hold/upgrade", post(handlers::hold::upgrade_building))
        .route("/skill/unlock", post(handlers::hold::unlock_skill))
        .route("/inventory/equip", post(handlers::inventory::equip_item))
        .route("/inventory/unequip", post(handlers::inventory::unequip_item))
        .route("/market/buy-item", post(handlers::market::buy_item))
        .route("/market/buy-consumable", post(handlers::market::buy_consumable))
        .route("/game/mission-types", get(handlers::game_data::get_mission_types))
        .route("/game/item-types", get(handlers::game_data::get_item_types))
        .route("/game/class-types", get(handlers::game_data::get_class_types))
        .route("/game/consumable-types", get(handlers::game_data::get_consumable_types))
        .route("/game/building-types", get(handlers::game_data::get_building_types))
}

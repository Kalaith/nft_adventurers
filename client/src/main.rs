//! NFT Adventurers - Game Client
//!
//! A real-time idle RPG with cryptographic ownership.

use macroquad::prelude::*;

mod api;
mod data;
mod game;
mod identity;
mod state;
mod ui;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "NFT Adventurers".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update().await;
        game.draw();  // Now takes &mut self
        next_frame().await;
    }
}

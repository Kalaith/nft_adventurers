//! Main menu and connecting screens.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use crate::game::PendingAction;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

/// Draw the main menu screen.
pub fn draw_main_menu() -> Option<PendingAction> {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    let title = "NFT Adventurers";
    let title_size = 48.0;
    let title_dims = measure_ui_text(title, None, title_size as u16, 1.0);
    draw_ui_text(
        title,
        center_x - title_dims.width / 2.0,
        center_y - 100.0,
        title_size,
        dark::TEXT_BRIGHT,
    );

    let subtitle = "Legends Forged in Chain";
    let sub_size = 24.0;
    let sub_dims = measure_ui_text(subtitle, None, sub_size as u16, 1.0);
    draw_ui_text(
        subtitle,
        center_x - sub_dims.width / 2.0,
        center_y - 60.0,
        sub_size,
        dark::TEXT_DIM,
    );

    if macroquad_toolkit::ui::button(center_x - 100.0, center_y, 200.0, 50.0, "Connect Wallet") {
        return Some(PendingAction::Connect);
    }

    None
}

/// Draw the connecting screen.
pub fn draw_connecting() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    draw_ui_text("Connecting...", center_x - 60.0, center_y, 24.0, dark::TEXT);
}

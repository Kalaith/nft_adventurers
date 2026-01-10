//! Hold and upgrade screens.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;

use shared::PlayerData;
use crate::game::PendingAction;

/// Draw the main hold screen.
pub fn draw(
    player_data: Option<&PlayerData>,
    assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;
    
    // Draw background
    if let Some(tex) = assets.get_texture("ui_background_main") {
        let scale_x = screen_width() / tex.width();
        let scale_y = screen_height() / tex.height();
        let scale = scale_x.max(scale_y);
        draw_texture_ex(
            tex,
            0.0,
            0.0,
            Color::new(1.0, 1.0, 1.0, 0.3),
            DrawTextureParams {
                dest_size: Some(vec2(tex.width() * scale, tex.height() * scale)),
                ..Default::default()
            },
        );
    }
    
    // Semi-transparent overlay
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.05, 0.05, 0.1, 0.7));

    let padding = 20.0;
    let panel_width = screen_width() - padding * 2.0;
    
    // Title bar
    draw_rectangle(padding, padding, panel_width, 50.0, Color::new(0.15, 0.12, 0.2, 0.9));
    draw_rectangle_lines(padding, padding, panel_width, 50.0, 2.0, Color::new(0.6, 0.5, 0.8, 0.5));
    draw_text("Your Hold", padding + 15.0, padding + 35.0, 32.0, Color::new(0.9, 0.85, 1.0, 1.0));

    let y = padding + 70.0;

    if let Some(data) = player_data {
        // Adventurers panel
        let adv_panel_height = 30.0 + (data.adventurers.len() as f32 * 80.0);
        draw_rectangle(padding, y, panel_width * 0.55, adv_panel_height, Color::new(0.1, 0.1, 0.15, 0.85));
        draw_rectangle_lines(padding, y, panel_width * 0.55, adv_panel_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
        
        draw_text("⚔ Adventurers", padding + 15.0, y + 22.0, 20.0, Color::new(0.7, 0.8, 1.0, 1.0));
        
        let mut card_y = y + 35.0;
        for adv in &data.adventurers {
            let card_x = padding + 10.0;
            let card_width = panel_width * 0.55 - 20.0;
            let card_height = 70.0;
            
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
            if let Some(tex) = assets.get_texture(&portrait_key) {
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
            
            // Info
            let text_x = card_x + 75.0;
            // Capitalize class display name
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
            
            let (status_str, status_color) = match &adv.status {
                shared::AdventurerStatus::Healthy => ("Ready", Color::new(0.3, 0.8, 0.3, 1.0)),
                shared::AdventurerStatus::OnMission { .. } => ("On Mission", Color::new(0.9, 0.7, 0.2, 1.0)),
                shared::AdventurerStatus::Injured { .. } => ("Injured", Color::new(0.9, 0.4, 0.3, 1.0)),
                shared::AdventurerStatus::Dead => ("Dead", Color::new(0.5, 0.5, 0.5, 1.0)),
            };
            draw_text(status_str, text_x, card_y + 60.0, 14.0, status_color);
            
            if macroquad_toolkit::ui::button(card_x + card_width - 90.0, card_y + 25.0, 80.0, 28.0, "View") {
                action = Some(PendingAction::GoToAdventurerDetail(adv.id.to_string()));
            }
            
            card_y += card_height + 8.0;
        }

        // Missions panel
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
                        action = Some(PendingAction::ResolveMission(mission.id.to_string()));
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
        action = Some(PendingAction::GoToMissions);
    }
    if macroquad_toolkit::ui::button(padding + 110.0, btn_y, 100.0, 40.0, "🏰 Upgrades") {
        action = Some(PendingAction::GoToHoldUpgrades);
    }
    if macroquad_toolkit::ui::button(padding + 220.0, btn_y, 100.0, 40.0, "📦 Inventory") {
        action = Some(PendingAction::GoToInventory);
    }

    // Show Recruit button if Tavern is built (level > 0) or simply available?
    // Let's assume unlocked if level >= 1
    if let Some(data) = player_data {
        if data.hold.building_level("tavern") > 0 {
             if macroquad_toolkit::ui::button(padding + 330.0, btn_y, 100.0, 40.0, "🍺 Recruit") {
                action = Some(PendingAction::GoToRecruit);
            }
        }
    }

    if macroquad_toolkit::ui::button(screen_width() - padding - 80.0, btn_y, 80.0, 40.0, "Logout") {
        action = Some(PendingAction::Disconnect);
    }
    
    action
}

/// Draw the building upgrades screen.
pub fn draw_upgrades(
    player_data: Option<&PlayerData>,
    assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;
    let mut y = 40.0;

    draw_text("Building Upgrades", 20.0, y, 32.0, dark::TEXT_BRIGHT);
    y += 50.0;

    let buildings = [
        ("hearth", "building_hearth", "Hearth", "+5% XP per level"),
        ("training_yard", "building_training_yard", "Training Yard", "Faster adventurer recovery"),
        ("feat_anvil", "building_feat_anvil", "Feat Anvil", "+8% XP per level"),
        ("tavern", "building_tavern", "Tavern", "Recruit new adventurers"),
    ];

    if let Some(data) = player_data {
        for (id, tex_key, name, desc) in buildings {
            // Draw building image
            if let Some(tex) = assets.get_texture(tex_key) {
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
                    action = Some(PendingAction::UpgradeBuilding(id.to_string()));
                }
            } else {
                draw_text("MAX LEVEL", 110.0, y + 60.0, 14.0, dark::POSITIVE);
            }
            y += 100.0;
        }
    }

    if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
        action = Some(PendingAction::GoToHold);
    }
    
    action
}

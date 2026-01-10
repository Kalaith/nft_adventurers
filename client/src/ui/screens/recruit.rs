//! Recruitment screen.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;
use shared::{ClassTypeData, PlayerData};

use crate::game::PendingAction;

pub fn draw(
    player_data: Option<&PlayerData>,
    class_types: &[ClassTypeData],
    assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;
    let mut y = 40.0;

    draw_text("Recruit Adventurer", 20.0, y, 32.0, dark::TEXT_BRIGHT);
    
    // Get tavern level
    let tavern_level = player_data
        .map(|d| d.hold.building_level("tavern"))
        .unwrap_or(0);

    draw_text(&format!("Tavern Level: {}", tavern_level), 350.0, y, 20.0, dark::ACCENT);
    
    y += 50.0;

    if class_types.is_empty() {
        draw_text("Loading classes...", 40.0, y, 20.0, dark::TEXT_DIM);
    } else {
        for class in class_types {
            let is_unlocked = tavern_level >= class.unlock_level;
            
            // Card background
            let card_height = 80.0;
            let card_color = if is_unlocked {
                Color::new(0.1, 0.15, 0.2, 0.8)
            } else {
                Color::new(0.1, 0.1, 0.1, 0.5)
            };
            
            draw_rectangle(20.0, y, 500.0, card_height, card_color);
            draw_rectangle_lines(20.0, y, 500.0, card_height, 1.0, Color::new(0.3, 0.3, 0.4, 0.5));

            // Portrait
            if let Some(tex) = assets.get_texture(&class.portrait_key) {
                let portrait_size = 64.0;
                let scale = portrait_size / tex.height();
                draw_texture_ex(
                    tex,
                    30.0,
                    y + 8.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(tex.width() * scale, portrait_size)),
                        ..Default::default()
                    },
                );
            }

            // Info
            draw_text(&class.display_name, 110.0, y + 25.0, 24.0, WHITE);
            
            // Stats preview
            let stats_text = format!("Str {} • Dex {} • Int {} • Con {} • Cha {}", 
                class.base_str, class.base_dex, class.base_int, class.base_con, class.base_cha);
            draw_text(&stats_text, 110.0, y + 45.0, 14.0, dark::TEXT_DIM);
            
            // Cost
            if class.cost > 0 {
                 draw_text(&format!("Cost: {} Gold", class.cost), 110.0, y + 65.0, 14.0, Color::new(1.0, 0.8, 0.2, 1.0));
            } else {
                 draw_text("Free", 110.0, y + 65.0, 14.0, Color::new(0.5, 0.8, 0.5, 1.0));
            }

            // Action
            if is_unlocked {
                if macroquad_toolkit::ui::button(400.0, y + 25.0, 100.0, 30.0, "Recruit") {
                    // For now, generate a random name or prompt? 
                    // To keep it simple, we'll auto-generate a generic name or use the class name + ID logic in backend?
                    // The backend expects a name. Let's just use "New [Class]" for now.
                    let name = format!("New {}", class.display_name);
                    action = Some(PendingAction::RecruitAdventurer { 
                        class_key: class.type_key.clone(), 
                        name 
                    });
                }
            } else {
                draw_text(&format!("Unlock Lv.{}", class.unlock_level), 400.0, y + 45.0, 16.0, dark::NEGATIVE);
            }

            y += card_height + 15.0;
        }
    }

    if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
        action = Some(PendingAction::GoToHold);
    }

    action
}

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;
use macroquad_toolkit::ui::button;
use shared::{ItemTypeData, ConsumableTypeData, PlayerData};
use crate::game::PendingAction;

const COMMON_PANEL_WIDTH: f32 = 400.0;

pub fn draw_smithy(
    player_data: Option<&PlayerData>,
    item_types: &[ItemTypeData],
    _assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;

    // Header metrics
    if let Some(data) = player_data {
        let metrics = format!("Gold: {}  Lumber: {}  Stone: {}", 
            data.hold.gold, data.hold.lumber, data.hold.stone);
        draw_text(&metrics, 20.0, 20.0, 20.0, dark::TEXT_DIM);
    }
    
    // Back button
    if button(20.0, 50.0, 100.0, 30.0, "Back to Hold") {
        return Some(PendingAction::GoToHold);
    }
    
    draw_text("Smithy - Buy Equipment", 20.0, 100.0, 30.0, dark::TEXT_BRIGHT);

    let start_y = 140.0;
    let mut y = start_y;
    let x = 20.0;
    
    if let Some(data) = player_data {
        for item in item_types {
            // Card background
            draw_rectangle(x, y, COMMON_PANEL_WIDTH, 70.0, Color::new(0.1, 0.1, 0.12, 0.9));
            draw_rectangle_lines(x, y, COMMON_PANEL_WIDTH, 70.0, 1.0, Color::new(0.3, 0.3, 0.4, 0.5));

            // Icon placeholder
            draw_rectangle(x + 10.0, y + 10.0, 50.0, 50.0, Color::new(0.05, 0.05, 0.08, 0.5));
            draw_text(&item.icon_key[..2.min(item.icon_key.len())], x + 25.0, y + 40.0, 20.0, dark::TEXT_DIM);

            // Info
            draw_text(&item.display_name, x + 70.0, y + 25.0, 20.0, dark::TEXT_BRIGHT);
            
            let stats = if let Some(dmg) = item.base_damage {
                 format!("Dmg: {}", dmg)
            } else if let Some(def) = item.base_defense {
                 format!("Def: {}", def)
            } else if let Some(heal) = item.base_healing {
                 format!("Heal: {}", heal)
            } else {
                 "Stats: ?".to_string()
            };
            draw_text(&stats, x + 70.0, y + 45.0, 16.0, dark::TEXT_DIM);

            // Buy Button
            let cost = item.cost;
            let can_afford = data.hold.gold >= cost;
            
            let btn_text = if can_afford {
                format!("Buy ({}g)", cost)
            } else {
                format!("Need {}g", cost)
            };
            
            if button(x + 250.0, y + 20.0, 120.0, 30.0, &btn_text) {
                if can_afford {
                    action = Some(PendingAction::BuyItem(item.type_key.clone()));
                }
            }
            
            y += 80.0;
        }
    } else {
        draw_text("Loading...", 20.0, 140.0, 20.0, dark::TEXT_DIM);
    }
    
    action
}

pub fn draw_market(
    player_data: Option<&PlayerData>,
    consumable_types: &[ConsumableTypeData],
    _assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;

    // Header metrics
    if let Some(data) = player_data {
        let metrics = format!("Gold: {}  Lumber: {}  Stone: {}", 
            data.hold.gold, data.hold.lumber, data.hold.stone);
        draw_text(&metrics, 20.0, 20.0, 20.0, dark::TEXT_DIM);
    }
    
    // Back button
    if button(20.0, 50.0, 100.0, 30.0, "Back to Hold") {
        return Some(PendingAction::GoToHold);
    }
    
    draw_text("Market - Buy Consumables", 20.0, 100.0, 30.0, dark::TEXT_BRIGHT);

    let start_y = 140.0;
    let mut y = start_y;
    let x = 20.0;
    
    if let Some(data) = player_data {
        for cons in consumable_types {
            // Card background
            draw_rectangle(x, y, COMMON_PANEL_WIDTH, 90.0, Color::new(0.1, 0.1, 0.12, 0.9));
            draw_rectangle_lines(x, y, COMMON_PANEL_WIDTH, 90.0, 1.0, Color::new(0.3, 0.3, 0.4, 0.5));

            // Icon placeholder
            draw_rectangle(x + 10.0, y + 20.0, 50.0, 50.0, Color::new(0.05, 0.05, 0.08, 0.5));
            draw_text(&cons.icon_key[..2.min(cons.icon_key.len())], x + 25.0, y + 50.0, 20.0, dark::TEXT_DIM);

            // Info
            draw_text(&cons.display_name, x + 70.0, y + 25.0, 20.0, dark::TEXT_BRIGHT);
            draw_text(&cons.description, x + 70.0, y + 45.0, 14.0, dark::TEXT_DIM);

            // Buy Button
            let cost = cons.cost;
            let can_afford = data.hold.gold >= cost;
            
            let btn_text = if can_afford {
                format!("Buy ({}g)", cost)
            } else {
                format!("Need {}g", cost)
            };
            
            if button(x + 250.0, y + 30.0, 120.0, 30.0, &btn_text) {
                if can_afford {
                    action = Some(PendingAction::BuyConsumable(cons.type_key.clone()));
                }
            }
            
            y += 100.0;
        }
    } else {
        draw_text("Loading...", 20.0, 140.0, 20.0, dark::TEXT_DIM);
    }
    
    action
}

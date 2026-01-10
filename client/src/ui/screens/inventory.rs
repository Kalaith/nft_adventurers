//! Inventory screen.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;

use shared::PlayerData;
use crate::game::PendingAction;

/// Draw the inventory screen.
pub fn draw(
    player_data: Option<&PlayerData>,
    assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;
    
    // Draw background overlay
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.05, 0.05, 0.1, 0.95));

    let padding = 20.0;
    let panel_width = screen_width() - padding * 2.0;
    
    // Title bar
    draw_rectangle(padding, padding, panel_width, 50.0, Color::new(0.15, 0.12, 0.2, 0.9));
    draw_rectangle_lines(padding, padding, panel_width, 50.0, 2.0, Color::new(0.6, 0.5, 0.8, 0.5));
    draw_text("📦 Inventory", padding + 15.0, padding + 35.0, 32.0, Color::new(0.9, 0.85, 1.0, 1.0));

    let mut y = padding + 70.0;

    if let Some(data) = player_data {
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
                if let Some(tex) = assets.get_texture(&format!("item_{}", item.type_key)) {
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
        action = Some(PendingAction::GoToHold);
    }
    
    action
}

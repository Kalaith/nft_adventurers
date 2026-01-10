//! Adventurer detail screen.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;

use shared::{Item, PlayerData};
use crate::game::PendingAction;

/// Draw the adventurer detail screen.
pub fn draw(
    adventurer_id: &str,
    player_data: Option<&PlayerData>,
    assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;
    
    // Draw background
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.05, 0.05, 0.1, 0.95));

    let padding = 20.0;
    let panel_width = screen_width() - padding * 2.0;

    // Find adventurer
    let adventurer = player_data
        .and_then(|data| {
            data.adventurers
                .iter()
                .find(|a| a.id.to_string() == adventurer_id)
        });

    let adv = match adventurer {
        Some(a) => a,
        None => {
            draw_text("Adventurer not found", padding, 80.0, 24.0, dark::NEGATIVE);
            if macroquad_toolkit::ui::button(padding, screen_height() - 60.0, 100.0, 40.0, "← Back") {
                return Some(PendingAction::GoToHold);
            }
            return None;
        }
    };

    // --- Header ---
    draw_rectangle(padding, padding, panel_width, 80.0, Color::new(0.15, 0.12, 0.2, 0.9));
    draw_rectangle_lines(padding, padding, panel_width, 80.0, 2.0, Color::new(0.6, 0.5, 0.8, 0.5));
    
    // Portrait
    let portrait_key = format!("portrait_{}", adv.class_key);
    if let Some(tex) = assets.get_texture(&portrait_key) {
        let portrait_size = 70.0;
        let scale = portrait_size / tex.height();
        draw_texture_ex(
            tex,
            padding + 5.0,
            padding + 5.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(tex.width() * scale, portrait_size)),
                ..Default::default()
            },
        );
    }
    
    // Name and class - capitalize class_key
    let class_str = {
        let mut s = adv.class_key.clone();
        if let Some(c) = s.get_mut(0..1) {
            c.make_ascii_uppercase();
        }
        s
    };
    draw_text(&adv.name, padding + 85.0, padding + 35.0, 28.0, WHITE);
    draw_text(
        &format!("{} • Level {}", class_str, adv.level),
        padding + 85.0,
        padding + 58.0,
        16.0,
        Color::new(0.7, 0.7, 0.7, 1.0),
    );

    let mut y = padding + 100.0;

    // --- Stats Panel ---
    let stats_height = 120.0;
    draw_rectangle(padding, y, panel_width * 0.4, stats_height, Color::new(0.1, 0.1, 0.15, 0.85));
    draw_rectangle_lines(padding, y, panel_width * 0.4, stats_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
    
    draw_text("📊 Stats", padding + 10.0, y + 22.0, 18.0, Color::new(0.7, 0.8, 1.0, 1.0));
    draw_text(&format!("STR: {}", adv.stats.str_), padding + 15.0, y + 50.0, 14.0, WHITE);
    draw_text(&format!("DEX: {}", adv.stats.dex), padding + 15.0, y + 68.0, 14.0, WHITE);
    draw_text(&format!("INT: {}", adv.stats.int), padding + 15.0, y + 86.0, 14.0, WHITE);
    draw_text(&format!("CON: {}", adv.stats.con), padding + 15.0, y + 104.0, 14.0, WHITE);
    
    draw_text(&format!("XP: {}", adv.xp), padding + 120.0, y + 50.0, 14.0, Color::new(0.8, 0.8, 0.3, 1.0));

    // --- Equipment Panel ---
    let equip_x = padding + panel_width * 0.42;
    let equip_width = panel_width * 0.58;
    draw_rectangle(equip_x, y, equip_width, stats_height, Color::new(0.1, 0.1, 0.15, 0.85));
    draw_rectangle_lines(equip_x, y, equip_width, stats_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
    
    draw_text("⚔ Equipment", equip_x + 10.0, y + 22.0, 18.0, Color::new(0.7, 0.8, 1.0, 1.0));

    // Get equipped items
    let items: Vec<&Item> = player_data
        .map(|d| d.items.iter().filter(|i| i.equipped_by == Some(adv.id)).collect())
        .unwrap_or_default();

    let slots = [
        ("Weapon", shared::EquipSlot::Weapon),
        ("Armor", shared::EquipSlot::Armor),
        ("Accessory", shared::EquipSlot::Accessory),
    ];

    let mut slot_y = y + 40.0;
    for (slot_name, slot) in slots {
        let equipped_item = items.iter().find(|i| i.equip_slot == slot);
        
        draw_text(
            &format!("{}: ", slot_name),
            equip_x + 15.0,
            slot_y,
            14.0,
            Color::new(0.6, 0.6, 0.8, 1.0),
        );
        
        if let Some(item) = equipped_item {
            draw_text(&item.current_name, equip_x + 90.0, slot_y, 14.0, WHITE);
            
            // Unequip button
            if macroquad_toolkit::ui::button(equip_x + equip_width - 80.0, slot_y - 12.0, 60.0, 20.0, "Remove") {
                action = Some(PendingAction::UnequipSlot {
                    adventurer_id: adventurer_id.to_string(),
                    slot: slot_name.to_lowercase(),
                });
            }
        } else {
            draw_text("(empty)", equip_x + 90.0, slot_y, 14.0, Color::new(0.5, 0.5, 0.5, 1.0));
        }
        
        slot_y += 26.0;
    }

    y += stats_height + 15.0;

    // --- Available Items to Equip ---
    let items_height = 180.0;
    draw_rectangle(padding, y, panel_width, items_height, Color::new(0.1, 0.1, 0.15, 0.85));
    draw_rectangle_lines(padding, y, panel_width, items_height, 1.0, Color::new(0.4, 0.4, 0.6, 0.5));
    
    draw_text("📦 Available Items (click to equip)", padding + 15.0, y + 22.0, 18.0, Color::new(0.8, 0.7, 1.0, 1.0));

    if let Some(data) = player_data {
        let unequipped: Vec<&Item> = data.items.iter().filter(|i| !i.is_equipped()).collect();
        
        if unequipped.is_empty() {
            draw_text("No unequipped items", padding + 15.0, y + 60.0, 14.0, Color::new(0.5, 0.5, 0.5, 1.0));
        } else {
            let mut item_x = padding + 15.0;
            let mut item_y = y + 40.0;
            let item_size = 50.0;
            
            for item in unequipped.iter().take(8) {
                let rarity_color = match item.rarity {
                    shared::Rarity::Common => Color::new(0.3, 0.3, 0.3, 0.8),
                    shared::Rarity::Uncommon => Color::new(0.2, 0.4, 0.2, 0.8),
                    shared::Rarity::Rare => Color::new(0.2, 0.3, 0.5, 0.8),
                    shared::Rarity::Epic => Color::new(0.4, 0.2, 0.5, 0.8),
                    shared::Rarity::Legendary => Color::new(0.5, 0.4, 0.1, 0.8),
                    shared::Rarity::Mythic => Color::new(0.5, 0.2, 0.2, 0.8),
                };
                
                // Clickable item card
                let mx = mouse_position().0;
                let my = mouse_position().1;
                let hovered = mx >= item_x && mx <= item_x + item_size && my >= item_y && my <= item_y + item_size + 20.0;
                
                let bg_color = if hovered {
                    Color::new(rarity_color.r + 0.1, rarity_color.g + 0.1, rarity_color.b + 0.1, 0.95)
                } else {
                    rarity_color
                };
                
                draw_rectangle(item_x, item_y, item_size, item_size + 20.0, bg_color);
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
                
                // Slot indicator
                let slot_char = match item.equip_slot {
                    shared::EquipSlot::Weapon => "W",
                    shared::EquipSlot::Armor => "A",
                    shared::EquipSlot::Accessory => "R",
                };
                draw_text(slot_char, item_x + 2.0, item_y + item_size + 14.0, 10.0, WHITE);
                
                // Click to equip
                if hovered && is_mouse_button_pressed(MouseButton::Left) {
                    action = Some(PendingAction::EquipItem {
                        adventurer_id: adventurer_id.to_string(),
                        item_id: item.id.to_string(),
                    });
                }
                
                item_x += item_size + 10.0;
                if item_x + item_size > padding + panel_width - 15.0 {
                    item_x = padding + 15.0;
                    item_y += item_size + 25.0;
                }
            }
        }
    }

    // --- Bottom Buttons ---
    if macroquad_toolkit::ui::button(padding, screen_height() - 60.0, 100.0, 40.0, "← Back") {
        action = Some(PendingAction::GoToHold);
    }
    
    if macroquad_toolkit::ui::button(padding + 110.0, screen_height() - 60.0, 100.0, 40.0, "Skills") {
        action = Some(PendingAction::GoToSkills(adventurer_id.to_string()));
    }
    
    action
}

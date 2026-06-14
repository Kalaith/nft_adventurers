//! Mission select screen.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;

use crate::game::PendingAction;
use macroquad_toolkit::ui::draw_ui_text;
use shared::PlayerData;

/// Draw the mission select screen.
pub fn draw(
    player_data: Option<&PlayerData>,
    selected_adventurer: &Option<String>,
    assets: &AssetManager,
) -> Option<PendingAction> {
    let mut action = None;
    let mut y = 40.0;

    draw_ui_text("Select Mission", 20.0, y, 32.0, dark::TEXT_BRIGHT);
    y += 40.0;

    // Adventurer selection
    draw_ui_text("Adventurer:", 20.0, y, 20.0, dark::ACCENT);
    y += 25.0;

    let mut available = Vec::new();
    if let Some(data) = player_data {
        for adv in &data.adventurers {
            if adv.is_available() {
                available.push((adv.id.to_string(), adv.name.clone(), adv.level));
            }
        }
    }

    if available.is_empty() {
        draw_ui_text("No adventurers available!", 30.0, y, 18.0, dark::NEGATIVE);
        y += 30.0;
    } else {
        for (id, name, level) in &available {
            let selected = selected_adventurer.as_ref() == Some(id);
            let label = if selected {
                format!("✓ {} Lv.{}", name, level)
            } else {
                format!("  {} Lv.{}", name, level)
            };

            if macroquad_toolkit::ui::button(30.0, y, 200.0, 28.0, &label) {
                action = Some(PendingAction::SelectAdventurer(id.clone()));
            }
            y += 32.0;
        }
    }

    y += 20.0;
    draw_ui_text("Mission:", 20.0, y, 20.0, dark::ACCENT);
    y += 30.0;

    let missions = [
        shared::MissionType::QuickSkirmish,
        shared::MissionType::DungeonCrawl,
        shared::MissionType::BossRaid,
    ];

    let can_start = selected_adventurer.is_some();

    for mission_type in missions {
        // Draw mission thumbnail
        if let Some(tex) = assets.get_texture(mission_type.icon_key()) {
            let thumb_height = 60.0;
            let scale = thumb_height / tex.height();
            draw_texture_ex(
                tex,
                30.0,
                y - 5.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(tex.width() * scale, thumb_height)),
                    ..Default::default()
                },
            );
        }

        draw_ui_text(
            mission_type.display_name(),
            150.0,
            y + 20.0,
            18.0,
            dark::TEXT_BRIGHT,
        );
        draw_ui_text(
            mission_type.description(),
            150.0,
            y + 40.0,
            14.0,
            dark::TEXT_DIM,
        );

        // Hardcoded costs/rewards for display
        let (cost, reward) = match mission_type {
            shared::MissionType::QuickSkirmish => ("Free", "Reward: 10G, 5L"),
            shared::MissionType::DungeonCrawl => ("Cost: 20G", "Reward: 50G, 20L, 10S"),
            shared::MissionType::BossRaid => ("Cost: 100G", "Reward: 500G, 100L, 50S"),
        };

        draw_ui_text(
            cost,
            150.0,
            y + 58.0,
            14.0,
            if cost == "Free" {
                dark::POSITIVE
            } else {
                Color::new(1.0, 0.8, 0.2, 1.0)
            },
        );
        draw_ui_text(reward, 250.0, y + 58.0, 14.0, dark::ACCENT);

        if can_start {
            if macroquad_toolkit::ui::button(350.0, y + 15.0, 80.0, 28.0, "Go") {
                if let Some(adv_id) = selected_adventurer {
                    action = Some(PendingAction::StartMission {
                        mission_type: mission_type.type_key().to_string(),
                        adventurer_id: adv_id.clone(),
                    });
                }
            }
        }
        y += 70.0;
    }

    if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
        action = Some(PendingAction::GoToHold);
    }

    action
}

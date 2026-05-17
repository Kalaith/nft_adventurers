//! Skills screen.

use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;

use crate::game::PendingAction;
use shared::PlayerData;

/// Draw the skills screen for an adventurer.
pub fn draw(adventurer_id: &str, player_data: Option<&PlayerData>) -> Option<PendingAction> {
    let mut action = None;
    let mut y = 40.0;

    // Find adventurer
    let adventurer = player_data
        .and_then(|data| {
            data.adventurers
                .iter()
                .find(|a| a.id.to_string() == adventurer_id)
        })
        .cloned();

    let adv = match adventurer {
        Some(a) => a,
        None => {
            draw_text("Adventurer not found", 20.0, y, 24.0, dark::NEGATIVE);
            if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
                return Some(PendingAction::GoToHold);
            }
            return None;
        }
    };

    draw_text(
        &format!("{} - Skills", adv.name),
        20.0,
        y,
        32.0,
        dark::TEXT_BRIGHT,
    );
    y += 50.0;

    // Get skill tree for this class
    let skill_tree = match adv.class_key.as_str() {
        "warrior" => shared::SkillTree::warrior(),
        "mage" => shared::SkillTree::mage(),
        "cleric" => shared::SkillTree::cleric(),
        _ => shared::SkillTree::warrior(),
    };

    let current_tier = adv.skills.len() as u32;

    for node in &skill_tree.nodes {
        let unlocked = adv.skills.contains(&node.id);
        let can_unlock = !unlocked && node.tier == current_tier + 1;

        let color = if unlocked {
            dark::POSITIVE
        } else if can_unlock {
            dark::ACCENT
        } else {
            dark::TEXT_DIM
        };

        draw_text(
            &format!("[Tier {}] {}", node.tier, node.name),
            30.0,
            y,
            20.0,
            color,
        );
        y += 20.0;
        draw_text(&node.description, 30.0, y, 14.0, dark::TEXT_DIM);
        y += 20.0;

        if can_unlock {
            if macroquad_toolkit::ui::button(30.0, y, 100.0, 25.0, "Unlock") {
                action = Some(PendingAction::UnlockSkill {
                    adventurer_id: adventurer_id.to_string(),
                    skill_id: node.id.clone(),
                });
            }
            y += 30.0;
        } else if unlocked {
            draw_text("✓ Unlocked", 30.0, y, 14.0, dark::POSITIVE);
            y += 20.0;
        }
        y += 15.0;
    }

    if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
        action = Some(PendingAction::GoToHold);
    }

    action
}

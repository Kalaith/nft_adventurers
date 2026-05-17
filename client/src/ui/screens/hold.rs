//! Hold and upgrade screens.

use macroquad::prelude::*;
use macroquad_toolkit::assets::AssetManager;
use macroquad_toolkit::colors::dark;

use crate::game::PendingAction;
use shared::PlayerData;

/// Draw the main hold screen.
pub fn draw(player_data: Option<&PlayerData>, assets: &AssetManager) -> Option<PendingAction> {
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
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.05, 0.05, 0.1, 0.7),
    );

    let padding = 20.0;
    let panel_width = screen_width() - padding * 2.0;

    // Title bar
    crate::ui::draw_title_surface(Rect::new(padding, padding, panel_width, 50.0));
    draw_text(
        "Your Hold",
        padding + 15.0,
        padding + 35.0,
        32.0,
        Color::new(0.9, 0.85, 1.0, 1.0),
    );

    if let Some(data) = player_data {
        let res_text = format!(
            "🪙 {}  🌲 {}  🪨 {}",
            data.hold.gold, data.hold.lumber, data.hold.stone
        );
        let res_size = measure_text(&res_text, None, 20, 1.0);
        draw_text(
            &res_text,
            screen_width() - padding - res_size.width - 20.0,
            padding + 32.0,
            20.0,
            Color::new(1.0, 0.9, 0.5, 1.0),
        );
    }

    let y = padding + 70.0;

    if let Some(data) = player_data {
        // Adventurers panel
        let adv_panel_height = 30.0 + (data.adventurers.len() as f32 * 80.0);
        crate::ui::draw_content_surface(Rect::new(
            padding,
            y,
            panel_width * 0.55,
            adv_panel_height,
        ));

        draw_text(
            "⚔ Adventurers",
            padding + 15.0,
            y + 22.0,
            20.0,
            Color::new(0.7, 0.8, 1.0, 1.0),
        );

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
            crate::ui::draw_surface_panel(
                Rect::new(card_x, card_y, card_width, card_height),
                card_color,
                Color::new(0.5, 0.5, 0.5, 0.4),
                1.0,
            );

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
                shared::AdventurerStatus::OnMission { .. } => {
                    ("On Mission", Color::new(0.9, 0.7, 0.2, 1.0))
                }
                shared::AdventurerStatus::Injured { .. } => {
                    ("Injured", Color::new(0.9, 0.4, 0.3, 1.0))
                }
                shared::AdventurerStatus::Dead => ("Dead", Color::new(0.5, 0.5, 0.5, 1.0)),
            };
            draw_text(status_str, text_x, card_y + 60.0, 14.0, status_color);

            if macroquad_toolkit::ui::button(
                card_x + card_width - 90.0,
                card_y + 25.0,
                80.0,
                28.0,
                "View",
            ) {
                action = Some(PendingAction::GoToAdventurerDetail(adv.id.to_string()));
            }

            card_y += card_height + 8.0;
        }

        // Missions panel
        let missions_x = padding + panel_width * 0.57;
        let missions_width = panel_width * 0.43;
        let missions_height = 30.0 + (data.active_missions.len() as f32 * 50.0).max(60.0);

        draw_rectangle(
            missions_x,
            y,
            missions_width,
            missions_height,
            Color::new(0.1, 0.1, 0.15, 0.85),
        );
        draw_rectangle_lines(
            missions_x,
            y,
            missions_width,
            missions_height,
            1.0,
            Color::new(0.4, 0.4, 0.6, 0.5),
        );

        draw_text(
            "⏱ Active Missions",
            missions_x + 15.0,
            y + 22.0,
            20.0,
            Color::new(0.9, 0.8, 0.5, 1.0),
        );

        let mut mission_y = y + 40.0;
        if data.active_missions.is_empty() {
            draw_text(
                "No active missions",
                missions_x + 15.0,
                mission_y + 15.0,
                14.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            );
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
                draw_text(
                    &time_str,
                    missions_x + missions_width - 100.0,
                    mission_y,
                    14.0,
                    time_color,
                );

                if is_complete {
                    if macroquad_toolkit::ui::button(
                        missions_x + missions_width - 85.0,
                        mission_y + 5.0,
                        70.0,
                        24.0,
                        "Claim",
                    ) {
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
    draw_rectangle(
        0.0,
        nav_y,
        screen_width(),
        70.0,
        Color::new(0.08, 0.08, 0.12, 0.95),
    );
    draw_line(
        0.0,
        nav_y,
        screen_width(),
        nav_y,
        2.0,
        Color::new(0.4, 0.3, 0.6, 0.5),
    );

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
        let mut btn_x = padding + 330.0;

        if data.hold.building_level("tavern") > 0 {
            if macroquad_toolkit::ui::button(btn_x, btn_y, 100.0, 40.0, "🍺 Recruit") {
                action = Some(PendingAction::GoToRecruit);
            }
            btn_x += 110.0;
        }

        if data.hold.building_level("smithy") > 0 {
            if macroquad_toolkit::ui::button(btn_x, btn_y, 100.0, 40.0, "⚒ Smithy") {
                action = Some(PendingAction::GoToSmithy);
            }
            btn_x += 110.0;
        }

        if data.hold.building_level("market") > 0 {
            if macroquad_toolkit::ui::button(btn_x, btn_y, 100.0, 40.0, "⚖ Market") {
                action = Some(PendingAction::GoToMarket);
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
    building_types: &[shared::BuildingTypeData],
    assets: &AssetManager,
    scroll: &mut f32,
) -> Option<PendingAction> {
    let mut action = None;

    draw_text("Building Upgrades", 20.0, 40.0, 32.0, dark::TEXT_BRIGHT);

    // Draw current resources at top for reference
    if let Some(data) = player_data {
        let res_text = format!(
            "Hold: {} Gold | {} Lumber | {} Stone",
            data.hold.gold, data.hold.lumber, data.hold.stone
        );
        draw_text(
            &res_text,
            screen_width() - 350.0,
            30.0,
            20.0,
            Color::new(1.0, 0.9, 0.5, 1.0),
        );
    }

    let start_y = 80.0;

    // Handle scrolling
    let (_, wheel_y) = mouse_wheel();
    *scroll -= wheel_y * 40.0; // Adjust scroll speed
    *scroll = scroll.max(0.0);

    if let Some(data) = player_data {
        if building_types.is_empty() {
            draw_text(
                "Loading building data...",
                40.0,
                start_y,
                20.0,
                dark::TEXT_DIM,
            );
        }

        let cols = 2;
        let padding = 15.0;
        let card_width = (screen_width() - padding * 3.0) / 2.0;
        let card_height = 140.0; // Increased height significantly

        // Calculate max scroll
        let rows = (building_types.len() + 1) / 2;
        let content_height = rows as f32 * (card_height + padding);
        let view_height = screen_height() - start_y - 80.0; // Reserve space for bottom button

        let max_scroll = (content_height - view_height).max(0.0);
        *scroll = scroll.min(max_scroll);

        // Apply scroll offset (view window)
        // We could use scissor test here, but for now we just draw everything relative to y-scroll

        for (i, building) in building_types.iter().enumerate() {
            let row = (i / cols) as f32;
            let col = (i % cols) as f32;

            let x = padding + col * (card_width + padding);
            let y = start_y + row * (card_height + padding) - *scroll;

            // Simple culling
            if y + card_height < start_y || y > screen_height() - 70.0 {
                continue;
            }

            // Background for building card
            draw_rectangle(
                x,
                y,
                card_width,
                card_height,
                Color::new(0.1, 0.1, 0.12, 0.8),
            );
            draw_rectangle_lines(
                x,
                y,
                card_width,
                card_height,
                1.0,
                Color::new(0.3, 0.3, 0.4, 0.5),
            );

            // Draw building image (smaller icon)
            if let Some(tex) = assets.get_texture(&building.icon_key) {
                let icon_size = 50.0;
                let scale = icon_size / tex.height();
                draw_texture_ex(
                    tex,
                    x + 10.0,
                    y + 10.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(tex.width() * scale, icon_size)),
                        ..Default::default()
                    },
                );
            }

            let level = data.hold.building_level(&building.type_key);
            let id = &building.type_key;

            draw_text(
                &format!("{} (Lv.{})", building.display_name, level),
                x + 70.0,
                y + 25.0,
                20.0,
                dark::TEXT_BRIGHT,
            );

            // Description wrapping? Just truncate for now or show first line
            let desc = if building.description.len() > 40 {
                format!("{}...", &building.description[..37])
            } else {
                building.description.clone()
            };
            draw_text(&desc, x + 70.0, y + 45.0, 14.0, dark::TEXT_DIM);

            if level < 5 {
                let factor = building.cost_scaling.powi(level as i32);
                let cost_g = (building.base_cost_gold as f32 * factor) as u32;
                let cost_l = (building.base_cost_lumber as f32 * factor) as u32;
                let cost_s = (building.base_cost_stone as f32 * factor) as u32;

                // Draw Cost Breakdown
                let mut cost_x = x + 10.0;
                let cost_y = y + 70.0;

                let draw_cost = |amount: u32, current: u32, label: &str, draw_x: f32| {
                    if amount > 0 {
                        let color = if current >= amount {
                            dark::POSITIVE
                        } else {
                            dark::NEGATIVE
                        };
                        draw_text(
                            &format!("{}: {}", label, amount),
                            draw_x,
                            cost_y,
                            14.0,
                            WHITE,
                        );
                        // Status indicator
                        draw_circle(
                            draw_x
                                + measure_text(&format!("{}: {}", label, amount), None, 14, 1.0)
                                    .width
                                + 5.0,
                            cost_y - 5.0,
                            3.0,
                            color,
                        );
                    }
                };

                draw_cost(cost_g, data.hold.gold, "Gold", cost_x);
                draw_cost(cost_l, data.hold.lumber, "Lum", cost_x + 90.0);
                draw_cost(cost_s, data.hold.stone, "Stn", cost_x + 180.0);

                // Check affordability (visual feedback)
                let can_afford = data.hold.gold >= cost_g
                    && data.hold.lumber >= cost_l
                    && data.hold.stone >= cost_s;
                let btn_text = if can_afford { "Upgrade" } else { "Need Res" };

                if macroquad_toolkit::ui::button(
                    x + card_width - 110.0,
                    y + card_height - 35.0,
                    100.0,
                    25.0,
                    btn_text,
                ) {
                    if can_afford {
                        action = Some(PendingAction::UpgradeBuilding(id.to_string()));
                    }
                }
            } else {
                draw_text(
                    "MAX LEVEL",
                    x + card_width - 100.0,
                    y + card_height - 25.0,
                    16.0,
                    dark::ACCENT,
                );
            }
        }
    }

    // Top overlay to cover scrolled content (optional, or rely on background clear)
    // Actually, background clear handles bottom items, but top items might draw over header.
    // Redraw header background to cover any scrolling overlap
    draw_rectangle(0.0, 0.0, screen_width(), 70.0, dark::BACKGROUND);
    draw_text("Building Upgrades", 20.0, 40.0, 32.0, dark::TEXT_BRIGHT);
    if let Some(data) = player_data {
        let res_text = format!(
            "Hold: {} Gold | {} Lumber | {} Stone",
            data.hold.gold, data.hold.lumber, data.hold.stone
        );
        draw_text(
            &res_text,
            screen_width() - 350.0,
            30.0,
            20.0,
            Color::new(1.0, 0.9, 0.5, 1.0),
        );
    }

    if macroquad_toolkit::ui::button(20.0, screen_height() - 60.0, 100.0, 40.0, "Back") {
        action = Some(PendingAction::GoToHold);
    }

    action
}

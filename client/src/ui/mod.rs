//! UI components and actions.

use macroquad::prelude::*;

pub mod screens;

pub fn draw_surface_panel(rect: Rect, fill: Color, border: Color, border_width: f32) {
    let surface = macroquad_toolkit::ui::SurfaceStyle::new(fill).with_border(border_width, border);
    macroquad_toolkit::ui::draw_surface(rect, &surface);
}

pub fn draw_title_surface(rect: Rect) {
    draw_surface_panel(
        rect,
        Color::new(0.15, 0.12, 0.2, 0.9),
        Color::new(0.6, 0.5, 0.8, 0.5),
        2.0,
    );
}

pub fn draw_content_surface(rect: Rect) {
    draw_surface_panel(
        rect,
        Color::new(0.1, 0.1, 0.15, 0.85),
        Color::new(0.4, 0.4, 0.6, 0.5),
        1.0,
    );
}

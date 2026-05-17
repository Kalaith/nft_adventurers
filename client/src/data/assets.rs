//! Asset configuration loading.

pub type TextureEntry = macroquad_toolkit::assets::TextureConfig;

pub struct TextureConfig;

impl TextureConfig {
    pub fn load_textures() -> Vec<TextureEntry> {
        let json_content = include_str!("../../assets/textures.json");
        let entries = macroquad_toolkit::assets::TextureConfig::from_json(json_content)
            .expect("Failed to parse textures.json");

        entries
            .into_iter()
            .map(|entry| entry.resolved(&["client/"]))
            .collect()
    }
}

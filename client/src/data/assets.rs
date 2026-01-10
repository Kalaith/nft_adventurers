//! Asset configuration loading.

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct TextureEntry {
    pub key: String,
    pub path: String,
}

pub struct TextureConfig;

impl TextureConfig {
    pub fn load_textures() -> Vec<TextureEntry> {
        #[derive(Deserialize)]
        struct Config(Vec<TextureEntry>);

        let json_content = include_str!("../../assets/textures.json");
        let entries: Vec<TextureEntry> = serde_json::from_str(json_content)
            .expect("Failed to parse textures.json");

        entries.into_iter()
            .map(|entry| {
                TextureEntry {
                    key: entry.key,
                    path: Self::resolve_path(&entry.path),
                }
            })
            .collect()
    }

    fn resolve_path(original_path: &str) -> String {
        if Path::new(original_path).exists() {
            return original_path.to_string();
        }

        if let Some(stripped) = original_path.strip_prefix("client/") {
            if Path::new(stripped).exists() {
                return stripped.to_string();
            }
        }

        original_path.to_string()
    }
}

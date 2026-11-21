use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub show_demo_boxes: bool,
    pub show_demo_names: bool,
    pub show_demo_health: bool,
    pub show_demo_distance: bool,
    pub max_demo_distance: f32,
    pub aim_simulation: bool,
    pub trigger_simulation: bool,
    pub vehicle_simulation: bool,
    pub vehicle_multiplier: f32,
    pub accent_r: u8,
    pub accent_g: u8,
    pub accent_b: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            show_demo_boxes: true,
            show_demo_names: true,
            show_demo_health: true,
            show_demo_distance: true,
            max_demo_distance: 250.0,
            aim_simulation: false,
            trigger_simulation: false,
            vehicle_simulation: false,
            vehicle_multiplier: 1.5,
            accent_r: 164,
            accent_g: 92,
            accent_b: 235,
        }
    }
}

impl AppConfig {
    fn path() -> PathBuf {
        let base = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("nightfall").join("config.json")
    }

    pub fn load() -> Self {
        fs::read(Self::path())
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();
        let parent = path.parent().ok_or_else(|| "Invalid config path".to_owned())?;
        fs::create_dir_all(parent).map_err(|e| format!("Create config directory: {e}"))?;
        let data = serde_json::to_vec_pretty(self).map_err(|e| format!("Serialize config: {e}"))?;
        fs::write(path, data).map_err(|e| format!("Write config: {e}"))
    }
}

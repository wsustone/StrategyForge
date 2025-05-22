use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// Game configuration settings
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    // Display settings
    pub resolution_width: f32,
    pub resolution_height: f32,
    pub fullscreen: bool,
    
    // Gameplay settings
    pub camera_speed: f32,
    pub camera_zoom_speed: f32,
    pub map_size: i32,
    pub strategic_points: i32,
    
    // Audio settings
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            // Display settings
            resolution_width: 1280.0,
            resolution_height: 720.0,
            fullscreen: false,
            
            // Gameplay settings
            camera_speed: 500.0,
            camera_zoom_speed: 1.5,
            map_size: 100,
            strategic_points: 3,
            
            // Audio settings
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.8,
        }
    }
}

/// Load configuration from file or create default
pub fn load_config() -> GameConfig {
    // In a real game, we would load from a file here
    // For now, just return the default config
    GameConfig::default()
}

/// Save configuration to file
pub fn save_config(_config: &GameConfig) -> Result<(), String> {
    // In a real game, we would save to a file here
    // For now, just pretend we saved it
    Ok(())
}

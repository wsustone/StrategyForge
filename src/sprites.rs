use bevy::prelude::*;
use std::collections::HashMap;

/// Resource to manage game sprites
#[derive(Resource, Default)]
pub struct GameSprites {
    // Map unit type to its sprite handles (for each direction)
    pub unit_sprites: HashMap<String, Vec<Handle<Image>>>,
    // Mobile base sprites
    pub base_sprites: HashMap<String, Vec<Handle<Image>>>,
    pub is_loaded: bool,
}

impl GameSprites {
    pub fn get_unit_sprite(&self, unit_type: &str, direction: usize) -> Option<&Handle<Image>> {
        self.unit_sprites.get(unit_type).and_then(|sprites| sprites.get(direction % 8))
    }
    
    pub fn get_base_sprite(&self, team: &str, direction: usize) -> Option<&Handle<Image>> {
        self.base_sprites.get(team).and_then(|sprites| sprites.get(direction % 8))
    }
}

pub struct SpriteLoaderPlugin;

impl Plugin for SpriteLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSprites>()
           .add_systems(Startup, load_sprites);
    }
}

fn load_sprites(
    asset_server: Res<AssetServer>,
    mut game_sprites: ResMut<GameSprites>,
) {
    info!("Loading unit and base sprites...");
    
    // Define the directions (0, 45, 90, 135, 180, 225, 270, 315 degrees)
    let directions = 8;
    
    // Load tank sprites for both teams
    let mut tank_sprites = Vec::with_capacity(directions);
    let mut enemy_tank_sprites = Vec::with_capacity(directions);
    
    // Load artillery sprites for both teams
    let mut artillery_sprites = Vec::with_capacity(directions);
    let mut enemy_artillery_sprites = Vec::with_capacity(directions);
    
    // Load base sprites for both teams
    let mut base_sprites = Vec::with_capacity(directions);
    let mut enemy_base_sprites = Vec::with_capacity(directions);
    
    // Load the sprites for each direction
    for angle in 0..directions {
        // Tank sprites - now in units subfolder
        let tank_path = format!("sprites/units/SimpleTank_rot{}.png", angle * 45);
        let tank_handle = asset_server.load(&tank_path);
        tank_sprites.push(tank_handle);
        
        // We use the same tank model but will tint it red for enemies in the spawning code
        enemy_tank_sprites.push(tank_sprites[angle].clone());
        
        // Artillery sprites - now in units subfolder
        let artillery_path = format!("sprites/units/SimpleArtillery_rot{}.png", angle * 45);
        let artillery_handle = asset_server.load(&artillery_path);
        artillery_sprites.push(artillery_handle);
        
        // We use the same artillery model but will tint it red for enemies in the spawning code
        enemy_artillery_sprites.push(artillery_sprites[angle].clone());
        
        // Base sprites - now in bases subfolder
        let base_path = format!("sprites/bases/SteampunkBase_rot{}.png", angle * 45);
        let base_handle = asset_server.load(&base_path);
        base_sprites.push(base_handle);
        
        // We use the same base model but will tint it red for enemies in the spawning code
        enemy_base_sprites.push(base_sprites[angle].clone());
    }
    
    // Store sprites in the resource
    game_sprites.unit_sprites.insert("tank_player".to_string(), tank_sprites);
    game_sprites.unit_sprites.insert("tank_enemy".to_string(), enemy_tank_sprites);
    game_sprites.unit_sprites.insert("artillery_player".to_string(), artillery_sprites);
    game_sprites.unit_sprites.insert("artillery_enemy".to_string(), enemy_artillery_sprites);
    
    game_sprites.base_sprites.insert("base_player".to_string(), base_sprites);
    game_sprites.base_sprites.insert("base_enemy".to_string(), enemy_base_sprites);
    
    game_sprites.is_loaded = true;
    info!("Sprite loading complete!");
}

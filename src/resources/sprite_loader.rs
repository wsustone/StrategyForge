use bevy::prelude::*;
use std::collections::HashMap;

/// Manages loading and storing sprite resources for the game
#[derive(Resource, Default)]
pub struct SpriteAssets {
    // Stores unit sprites by unit type and rotation angle
    unit_sprites: HashMap<String, HashMap<u16, Handle<Image>>>,
    // Stores resource sprites by resource type
    resource_sprites: HashMap<String, Handle<Image>>,
    // Stores base sprites by rotation angle
    base_sprites: HashMap<u16, Handle<Image>>,
    // Track loading state
    pub loaded: bool,
}

impl SpriteAssets {
    pub fn new() -> Self {
        Self {
            unit_sprites: HashMap::new(),
            resource_sprites: HashMap::new(),
            base_sprites: HashMap::new(),
            loaded: false,
        }
    }

    /// Get a unit sprite by unit type and rotation angle
    pub fn get_unit_sprite(&self, unit_type: &str, rotation_angle: u16) -> Option<&Handle<Image>> {
        self.unit_sprites
            .get(unit_type)
            .and_then(|angles| angles.get(&rotation_angle))
    }

    /// Get a resource sprite by resource type
    pub fn get_resource_sprite(&self, resource_type: &str) -> Option<&Handle<Image>> {
        self.resource_sprites.get(resource_type)
    }

    /// Get a base sprite by rotation angle
    pub fn get_base_sprite(&self, rotation_angle: u16) -> Option<&Handle<Image>> {
        self.base_sprites.get(&rotation_angle)
    }

    /// Add a unit sprite
    pub fn add_unit_sprite(&mut self, unit_type: &str, rotation_angle: u16, handle: Handle<Image>) {
        self.unit_sprites
            .entry(unit_type.to_string())
            .or_insert_with(HashMap::new)
            .insert(rotation_angle, handle);
    }

    /// Add a resource sprite
    pub fn add_resource_sprite(&mut self, resource_type: &str, handle: Handle<Image>) {
        self.resource_sprites.insert(resource_type.to_string(), handle);
    }

    /// Add a base sprite
    pub fn add_base_sprite(&mut self, rotation_angle: u16, handle: Handle<Image>) {
        self.base_sprites.insert(rotation_angle, handle);
    }
}

/// Plugin for loading all sprite assets
pub struct SpriteLoaderPlugin;

impl Plugin for SpriteLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, load_sprites);
    }
}

/// System to load all sprite assets
fn load_sprites(
    mut sprite_assets: ResMut<SpriteAssets>,
    asset_server: Res<AssetServer>,
) {
    // Load unit sprites
    load_unit_sprites(&mut sprite_assets, &asset_server);
    
    // Load resource sprites
    load_resource_sprites(&mut sprite_assets, &asset_server);
    
    // Load base sprites
    load_base_sprites(&mut sprite_assets, &asset_server);
    
    sprite_assets.loaded = true;
    println!("Sprite assets loaded successfully");
}

/// Load all unit sprites
fn load_unit_sprites(sprite_assets: &mut SpriteAssets, asset_server: &AssetServer) {
    // Define all unit types to load
    let unit_types = [
        "Tank",        // Basic tank
        "LandTank",    // Land-to-land tank
        "AATank",      // Land-to-air tank
        "Artillery",   // Artillery unit
        "LargeTank",   // Very large tank
        "Fighter",     // Air-to-air fighter
        "Bomber",      // Air-to-land bomber
        "LargeAircraft", // Very large hovering aircraft
        "Gatherer",    // Resource gatherer
        "Engineer",    // Engineer unit
    ];
    
    // Rotation angles (0, 45, 90, 135, 180, 225, 270, 315)
    let angles = [0, 45, 90, 135, 180, 225, 270, 315];
    
    for unit_type in unit_types.iter() {
        for angle in angles.iter() {
            let path = format!("sprites/units/{}_rot{}.png", unit_type, angle);
            let handle = asset_server.load(&path);
            sprite_assets.add_unit_sprite(unit_type, *angle, handle);
        }
    }
}

/// Load all resource sprites
fn load_resource_sprites(sprite_assets: &mut SpriteAssets, asset_server: &AssetServer) {
    // Define all resource types to load
    let resource_types = [
        "WoodDeposit",
        "StoneDeposit",
        "IronDeposit",
    ];
    
    for resource_type in resource_types.iter() {
        let path = format!("sprites/resources/{}_rot0.png", resource_type);
        let handle = asset_server.load(&path);
        sprite_assets.add_resource_sprite(resource_type, handle);
    }
}

/// Load all base sprites
fn load_base_sprites(sprite_assets: &mut SpriteAssets, asset_server: &AssetServer) {
    // Rotation angles (0, 45, 90, 135, 180, 225, 270, 315)
    let angles = [0, 45, 90, 135, 180, 225, 270, 315];
    
    for angle in angles.iter() {
        let path = format!("sprites/bases/SteampunkBase_rot{}.png", angle);
        let handle = asset_server.load(&path);
        sprite_assets.add_base_sprite(*angle, handle);
    }
}

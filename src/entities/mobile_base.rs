use bevy::prelude::*;
use crate::components::unit::{Team, UnitState};
use crate::components::unit_sprite::{IsometricSprite, SpriteCategory};
use crate::resources::sprite_loader::SpriteAssets;

/// Component for the mobile base - this is the player's main unit that must
/// reach and hold specific locations on the map
#[derive(Component)]
pub struct MobileBase {
    pub health: f32,
    pub max_health: f32,
    pub movement_speed: f32,
    pub team: Team,
    pub resources: BaseResources,
}

/// Resources stored in the mobile base
#[derive(Default)]
pub struct BaseResources {
    pub wood: f32,
    pub stone: f32,
    pub iron: f32,
}

impl Default for MobileBase {
    fn default() -> Self {
        Self {
            health: 1000.0,
            max_health: 1000.0,
            movement_speed: 1.0, // Slower than normal units
            team: Team::Player,
            resources: BaseResources::default(),
        }
    }
}

/// System to spawn the mobile base
pub fn spawn_mobile_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_assets: Option<Res<SpriteAssets>>,
) {
    // Only proceed if the SpriteAssets resource exists and is loaded
    let sprite_assets = match sprite_assets {
        Some(assets) if assets.loaded => assets,
        _ => return, // Exit early if resource doesn't exist or isn't loaded
    };
    
    // Use the sprite for the initial angle (0 degrees)
    let initial_angle: u16 = 0;
    let sprite_handle = if let Some(handle) = sprite_assets.get_base_sprite(initial_angle) {
        handle.clone()
    } else {
        // Fallback to a default sprite from the asset server
        asset_server.load("sprites/bases/SteampunkBase_rot0.png")
    };
    
    // Spawn the mobile base entity
    commands.spawn((
        SpriteBundle {
            texture: sprite_handle,
            transform: Transform::from_xyz(0.0, 0.0, 10.0)  // Z-layer above normal terrain
                .with_scale(Vec3::new(1.0, 1.0, 1.0)),      // Scale to match your game's scale
            ..Default::default()
        },
        MobileBase::default(),
        UnitState::Idle,
        IsometricSprite {
            sprite_type: "SteampunkBase".to_string(),
            angle: initial_angle,
            sprite_category: SpriteCategory::Base,
        },
        Name::new("Mobile Base"),
    ));
    
    info!("Mobile Steampunk Base spawned!");
}

/// Plugin for mobile base functionality
pub struct MobileBasePlugin;

impl Plugin for MobileBasePlugin {
    fn build(&self, app: &mut App) {
        // Use PostStartup to ensure this runs after SpriteAssets is initialized
        app.add_systems(PostStartup, spawn_mobile_base);
    }
}

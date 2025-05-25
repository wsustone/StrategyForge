use bevy::prelude::*;
use crate::resources::sprite_loader::SpriteAssets;

/// Component to link a unit with its isometric sprite type
#[derive(Component)]
pub struct IsometricSprite {
    /// The type of sprite (e.g., "Tank", "Fighter", "Artillery")
    pub sprite_type: String,
    /// Current rotation angle (0, 45, 90, 135, 180, 225, 270, 315)
    pub angle: u16,
    /// Whether this is a unit, resource, or base sprite
    pub sprite_category: SpriteCategory,
}

/// Category of sprite for determining which collection to look in
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SpriteCategory {
    Unit,
    Resource,
    Base,
}

impl Default for IsometricSprite {
    fn default() -> Self {
        Self {
            sprite_type: "Tank".to_string(),
            angle: 0,
            sprite_category: SpriteCategory::Unit,
        }
    }
}

/// System to update sprite based on entity rotation
pub fn update_unit_sprites(
    mut query: Query<(&mut Handle<Image>, &IsometricSprite, &Transform)>,
    sprite_assets: Option<Res<SpriteAssets>>,
) {
    // Only proceed if the SpriteAssets resource exists and is loaded
    let sprite_assets = match sprite_assets {
        Some(assets) if assets.loaded => assets,
        _ => return, // Exit early if resource doesn't exist or isn't loaded
    };
    
    for (mut image_handle, iso_sprite, transform) in query.iter_mut() {
        // Convert rotation to closest isometric angle
        let rotation_z = transform.rotation.to_euler(EulerRot::XYZ).2;
        let degrees = (rotation_z.to_degrees() + 360.0) % 360.0;
        
        // Find closest 45-degree increment
        let closest_angle = ((degrees / 45.0).round() * 45.0) % 360.0;
        let angle = closest_angle as u16;
        
        // Only update if we have a sprite for this angle
        match iso_sprite.sprite_category {
            SpriteCategory::Unit => {
                if let Some(sprite) = sprite_assets.get_unit_sprite(&iso_sprite.sprite_type, angle) {
                    *image_handle = sprite.clone();
                }
            },
            SpriteCategory::Resource => {
                if let Some(sprite) = sprite_assets.get_resource_sprite(&iso_sprite.sprite_type) {
                    *image_handle = sprite.clone();
                }
            },
            SpriteCategory::Base => {
                if let Some(sprite) = sprite_assets.get_base_sprite(angle) {
                    *image_handle = sprite.clone();
                }
            },
        }
    }
}

/// Plugin to handle isometric sprites
pub struct IsometricSpritePlugin;

impl Plugin for IsometricSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_unit_sprites);
    }
}

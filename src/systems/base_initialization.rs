use bevy::prelude::*;
use crate::components::player::MechanicalBase;
use crate::components::base_modules::{
    AttachmentPoint, 
    ModuleType,
    ResourceType,
};
use crate::states::game_state::GameState;

/// System to initialize attachment points on the mechanical base
pub fn initialize_base_attachments(
    mut commands: Commands,
    mut bases: Query<(Entity, &mut MechanicalBase)>, 
) {
    for (base_entity, mut base) in bases.iter_mut() {
        // Only initialize if no attachment points exist yet
        if base.attachment_points.is_empty() {
            // Create standard attachment points around the base
            let offsets = [
                (Vec2::new(0.0, 30.0), 0.0),   // Front
                (Vec2::new(0.0, -30.0), std::f32::consts::PI), // Back
                (Vec2::new(30.0, 0.0), std::f32::consts::PI / 2.0), // Right
                (Vec2::new(-30.0, 0.0), -std::f32::consts::PI / 2.0), // Left
            ];
            
            // Create attachment points for movement modules
            for (offset, rotation) in offsets.iter() {
                let point_entity = commands.spawn((
                    AttachmentPoint::new(
                        *offset,
                        *rotation,
                        Vec2::new(20.0, 20.0),
                        ModuleType::Movement { 
                            speed_modifier: 1.5, 
                            efficiency: 0.9,
                            terrain_penalty_reduction: 0.5
                        },
                    ),
                    Name::new("Movement Attachment"),
                )).id();
                
                base.add_attachment_point(point_entity);
                commands.entity(base_entity).add_child(point_entity);
            }
            
            // Add utility attachment points (smaller, more numerous)
            let util_offsets = [
                (Vec2::new(20.0, 20.0), std::f32::consts::PI / 4.0),
                (Vec2::new(-20.0, 20.0), -std::f32::consts::PI / 4.0),
                (Vec2::new(20.0, -20.0), std::f32::consts::PI * 3.0 / 4.0),
                (Vec2::new(-20.0, -20.0), -std::f32::consts::PI * 3.0 / 4.0),
            ];
            
            for (offset, rotation) in util_offsets.iter() {
                let point_entity = commands.spawn((
                    AttachmentPoint::new(
                        *offset,
                        *rotation,
                        Vec2::new(15.0, 15.0),
                        ModuleType::Storage {
                            capacity: 50,
                            resource_type: ResourceType::Iron,
                            passive_generation: 0.1, // Small passive generation of resources
                        },
                    ),
                    Name::new("Utility Attachment"),
                )).id();
                
                base.add_attachment_point(point_entity);
                commands.entity(base_entity).add_child(point_entity);
            }
            
            info!("Initialized {} attachment points for base", 
                 base.attachment_points.len());
        }
    }
}

/// Plugin for base initialization systems
pub struct BaseInitializationPlugin;

impl Plugin for BaseInitializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Gameplay),
            initialize_base_attachments
        );
    }
}

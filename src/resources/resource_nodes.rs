use bevy::prelude::*;

// Resource types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResourceType {
    Wood,
    Stone,
    Iron,
}

// Resource node component
#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount_remaining: i32,
    pub max_amount: i32,
}

// Resource node factory
#[derive(Default)]
pub struct ResourceNodeFactory;

impl ResourceNodeFactory {
    pub fn spawn_resource_node(&self, commands: &mut Commands, position: Vec2) -> Entity {
        let resource_type = match rand::random::<u8>() % 3 {
            0 => ResourceType::Wood,
            1 => ResourceType::Stone,
            _ => ResourceType::Iron,
        };
        
        let color = match resource_type {
            ResourceType::Wood => Color::srgb(0.6, 0.4, 0.2),
            ResourceType::Stone => Color::srgb(0.5, 0.5, 0.5),
            ResourceType::Iron => Color::srgb(0.6, 0.6, 0.7),
        };
        
        let max_amount = match resource_type {
            ResourceType::Wood => 1000,
            ResourceType::Stone => 800,
            ResourceType::Iron => 500,
        };
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(12.0, 12.0)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 1.0),
                ..default()
            },
            ResourceNode {
                resource_type,
                amount_remaining: max_amount,
                max_amount,
            },
        )).id()
    }
}

// Plugin to handle resource nodes
pub struct ResourceNodePlugin;

impl Plugin for ResourceNodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_resource_nodes);
    }
}

// System to set up initial resource nodes
fn setup_resource_nodes(mut commands: Commands) {
    info!("Setting up resource nodes...");
    
    let factory = ResourceNodeFactory::default();
    
    // Create some initial resource nodes at fixed positions
    // In a real game, this would be based on the map and possibly randomized
    factory.spawn_resource_node(&mut commands, Vec2::new(100.0, 100.0));
    factory.spawn_resource_node(&mut commands, Vec2::new(-100.0, 100.0));
    factory.spawn_resource_node(&mut commands, Vec2::new(100.0, -100.0));
    factory.spawn_resource_node(&mut commands, Vec2::new(-100.0, -100.0));
    
    info!("Resource nodes setup complete");
}

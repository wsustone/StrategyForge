use bevy::prelude::*;
use crate::states::game_state::GameState;

/// Plugin for managing cameras across different game states
pub struct CameraManagerPlugin;

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), cleanup_cameras)
           .add_systems(OnEnter(GameState::MainMenu), cleanup_cameras)
           .add_systems(OnEnter(GameState::Gameplay), cleanup_cameras)
           .add_systems(OnEnter(GameState::GameOver), cleanup_cameras)
           .add_systems(OnEnter(GameState::Victory), cleanup_cameras);
    }
}

/// Component to mark a camera as belonging to a specific game state
#[derive(Component)]
pub struct StateCamera(pub GameState);

/// System to clean up cameras when transitioning between states
fn cleanup_cameras(
    mut commands: Commands,
    cameras: Query<Entity, With<Camera>>,
) {
    for camera in cameras.iter() {
        commands.entity(camera).despawn_recursive();
    }
}

/// Spawn a camera for a specific game state
pub fn spawn_camera_for_state(
    commands: &mut Commands,
    state: GameState,
) -> Entity {
    let mut entity_commands = commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
        StateCamera(state),
    ));
    
    // Add GameCamera component for gameplay state
    if state == GameState::Gameplay {
        use crate::systems::camera::GameCamera;
        entity_commands.insert(GameCamera {
            speed: 500.0,
            zoom_speed: 0.1,
            min_zoom: 0.1,
            max_zoom: 3.0,
        });
    }
    
    entity_commands.id()
}

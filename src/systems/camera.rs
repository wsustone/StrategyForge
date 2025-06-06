use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::states::game_state::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Remove the setup_camera system to avoid spawning duplicate cameras
        // The CameraManagerPlugin will handle camera creation for all states
        app.add_systems(
                Update, 
                camera_movement.run_if(in_state(GameState::Gameplay))
            );
    }
}

#[derive(Component)]
pub struct GameCamera {
    pub speed: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &GameCamera), (With<Camera>, With<crate::systems::camera_manager::StateCamera>)>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let (mut transform, camera) = query.single_mut();
    
    // Camera panning with WASD or arrow keys
    let mut direction = Vec3::ZERO;
    
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    
    if direction != Vec3::ZERO {
        let movement = direction.normalize() * camera.speed * time.delta_seconds();
        transform.translation += movement;
    }
    
    // Camera zoom with mouse wheel
    let mut zoom_delta = 0.0;
    for event in mouse_wheel_events.read() {
        zoom_delta += event.y;
    }
    
    if zoom_delta != 0.0 {
        let zoom_factor = 1.0 - zoom_delta * camera.zoom_speed * 0.1;
        transform.scale = Vec3::splat((transform.scale.x * zoom_factor)
            .clamp(camera.min_zoom, camera.max_zoom));
    }
}

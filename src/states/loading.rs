use bevy::prelude::*;
use crate::GameState;
use crate::utils::font_loader::get_font_handle;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_loading)
           .add_systems(Update, check_loading.run_if(in_state(GameState::Loading)))
           .add_systems(OnExit(GameState::Loading), cleanup_loading);
    }
}

// Resources for loading state
#[derive(Resource)]
struct LoadingTimer(Timer);

fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a simple loading screen
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ..default()
    }, LoadingUI)).with_children(|parent| {
        // Loading text
        parent.spawn(TextBundle::from_section(
            "Loading...",
            TextStyle {
                font: get_font_handle(&asset_server),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ));
    });
    
    // Add a timer to simulate loading assets
    commands.insert_resource(LoadingTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn check_loading(
    time: Res<Time>,
    mut loading_timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Update the timer
    if loading_timer.0.tick(time.delta()).just_finished() {
        // When the timer finishes, transition to the main menu
        next_state.set(GameState::MainMenu);
    }
}

// Mark nodes for cleanup
#[derive(Component)]
struct LoadingUI;

// Fix the cleanup system for Bevy 0.14.0
fn cleanup_loading(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera2d>>,
    ui_query: Query<Entity, With<LoadingUI>>,
) {
    // Remove the loading UI and camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    commands.remove_resource::<LoadingTimer>();
}

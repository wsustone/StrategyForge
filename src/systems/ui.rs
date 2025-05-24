use bevy::prelude::*;
use crate::components::player::PlayerResources;
use crate::states::game_state::GameState;

// UI systems plugin
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), setup_ui)
           .add_systems(Update, update_resource_display.run_if(in_state(GameState::Gameplay)));
    }
}

// Component to mark UI elements
#[derive(Component)]
pub struct GameUI;

// Component for resource display
#[derive(Component)]
pub struct ResourceDisplay;

// System to set up the game UI
fn setup_ui(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Will implement UI setup later
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        GameUI,
        ResourceDisplay,
    ));
}

// System to update the resource display
fn update_resource_display(
    _player_resources: Res<PlayerResources>,
    _query: Query<&mut Text, With<ResourceDisplay>>,
) {
    // Will implement resource display update logic later
}

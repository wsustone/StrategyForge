use bevy::prelude::*;
use crate::components::player::MechanicalBase;
use crate::components::unit::{Selected, Team, UnitState};
use crate::GameState;

// Component to mark UI elements as part of the base action UI
#[derive(Component)]
pub struct BaseActionUI;

// Component for the different action buttons
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseAction {
    Build,
    Upgrade,
    Move,
    Stop,
    Fortify,
}

// Plugin for the base action UI
pub struct BaseActionUIPlugin;

impl Plugin for BaseActionUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_base_action_ui,
                handle_button_interactions,
            ).run_if(in_state(GameState::Gameplay)));
            
        info!("Base Action UI Plugin initialized");
    }
}

// System to show/hide the base action UI when a base is selected/deselected
fn update_base_action_ui(
    mut commands: Commands,
    base_query: Query<&MechanicalBase, With<Selected>>,
    ui_query: Query<Entity, With<BaseActionUI>>,
    asset_server: Res<AssetServer>,
) {
    let player_base_selected = base_query.iter().any(|base| base.team == Team::Player);
    
    // If a player base is selected and the UI doesn't exist, create it
    if player_base_selected && ui_query.is_empty() {
        spawn_base_action_ui(&mut commands, &asset_server);
    }
    // If no player base is selected but the UI exists, despawn it
    else if !player_base_selected && !ui_query.is_empty() {
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Function to spawn the base action UI
fn spawn_base_action_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Create the main container - a panel at the bottom of the screen
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    width: Val::Auto,
                    height: Val::Px(120.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Px(10.0),
                    column_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                // Use standard colors
                background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                border_color: Color::srgb(0.3, 0.3, 0.3).into(),
                ..default()
            },
            BaseActionUI,
            Name::new("Base Action UI"),
        ))
        .with_children(|parent| {
            // Create action buttons
            create_action_button(parent, asset_server, "Build", BaseAction::Build);
            create_action_button(parent, asset_server, "Upgrade", BaseAction::Upgrade);
            create_action_button(parent, asset_server, "Move", BaseAction::Move);
            create_action_button(parent, asset_server, "Stop", BaseAction::Stop);
            create_action_button(parent, asset_server, "Fortify", BaseAction::Fortify);
        });

    info!("Base action UI spawned");
}

// Helper function to create an action button
fn create_action_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: &str,
    action: BaseAction,
) {
    // Button size is consistent for all buttons
    let button_size = 100.0;
    
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(button_size),
                    height: Val::Px(button_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                ..default()
            },
            action.clone(),
            Name::new(format!("{} Button", label)),
        ))
        .with_children(|parent| {
            // Add text label
            parent.spawn(
                TextBundle::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgb(0.95, 0.95, 0.95),
                    },
                ).with_style(Style {
                    align_self: AlignSelf::Center,
                    ..default()
                }),
            );
        });
}

// System to handle button interactions
fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &BaseAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_entities: Query<(Entity, Option<&mut UnitState>), With<Selected>>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Change button color when pressed
                *color = Color::srgb(0.35, 0.75, 0.35).into(); // Green for pressed
                info!("Action {:?} selected", action);
                
                // Handle the different actions
                match action {
                    BaseAction::Build => {
                        info!("Build menu would appear here");
                    },
                    BaseAction::Upgrade => {
                        info!("Upgrade options would appear here");
                    },
                    BaseAction::Move => {
                        // Set selected bases to Moving state
                        for (_, state_opt) in selected_entities.iter_mut() {
                            if let Some(mut state) = state_opt {
                                *state = UnitState::Moving;
                            }
                        }
                        info!("Moving mode activated - right click to move the base");
                    },
                    BaseAction::Stop => {
                        // Set selected bases to Idle state
                        for (_, state_opt) in selected_entities.iter_mut() {
                            if let Some(mut state) = state_opt {
                                *state = UnitState::Idle;
                            }
                        }
                        info!("Base commanded to stop");
                    },
                    BaseAction::Fortify => {
                        info!("Base fortifying at current position");
                    },
                }
            }
            Interaction::Hovered => {
                // Change button color when hovered
                *color = Color::srgb(0.33, 0.33, 0.33).into();
            }
            Interaction::None => {
                // Reset button color when not interacting
                *color = Color::srgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

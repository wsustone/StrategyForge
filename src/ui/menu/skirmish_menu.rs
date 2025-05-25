use bevy::prelude::*;
use super::components::{MenuUI, create_button, create_title};

/// Plugin for the skirmish menu
pub struct SkirmishMenuPlugin;

impl Plugin for SkirmishMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenSkirmishMenuEvent>()
           .add_event::<CloseSkirmishMenuEvent>()
           .add_systems(Update, handle_open_skirmish_menu_event)
           .add_systems(Update, handle_skirmish_menu_buttons.run_if(resource_exists::<SkirmishMenuState>))
           .add_systems(Update, handle_close_skirmish_menu_event);
    }
}

/// Event to open the skirmish menu
#[derive(Event)]
pub struct OpenSkirmishMenuEvent;

/// Event to close the skirmish menu
#[derive(Event)]
pub struct CloseSkirmishMenuEvent;

/// State of the skirmish menu
#[derive(Resource)]
struct SkirmishMenuState;

/// Marker component for skirmish menu UI elements
#[derive(Component)]
struct SkirmishMenuUI;

/// Skirmish menu button types
#[derive(Component)]
enum SkirmishMenuButton {
    QuickMatch,
    CustomGame,
    ChallengeMode,
    Back,
}

/// Handle the open skirmish menu event
fn handle_open_skirmish_menu_event(
    mut commands: Commands,
    mut ev_open_skirmish: EventReader<OpenSkirmishMenuEvent>,
    asset_server: Res<AssetServer>,
    skirmish_state: Option<Res<SkirmishMenuState>>,
) {
    for _ in ev_open_skirmish.read() {
        // Only open skirmish menu if it's not already open
        if skirmish_state.is_none() {
            commands.insert_resource(SkirmishMenuState);
            
            // Placeholder for skirmish menu UI
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                        z_index: ZIndex::Global(10),
                        ..default()
                    },
                    SkirmishMenuUI,
                    MenuUI,
                ))
                .with_children(|parent| {
                    // Skirmish menu panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(600.0),
                                height: Val::Auto,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(20.0)),
                                row_gap: Val::Px(15.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Title
                            create_title(parent, "SKIRMISH", &asset_server, 40.0);
                            
                            // Quick Match button
                            create_button(parent, "Quick Match", SkirmishMenuButton::QuickMatch, &asset_server, 300.0, 50.0);
                            
                            // Custom Game button
                            create_button(parent, "Custom Game", SkirmishMenuButton::CustomGame, &asset_server, 300.0, 50.0);
                            
                            // Challenge Mode button
                            create_button(parent, "Challenge Mode", SkirmishMenuButton::ChallengeMode, &asset_server, 300.0, 50.0);
                            
                            // Back button
                            create_button(parent, "Back", SkirmishMenuButton::Back, &asset_server, 300.0, 50.0);
                        });
                });
        }
    }
}

/// Handle the close skirmish menu event
fn handle_close_skirmish_menu_event(
    mut commands: Commands,
    mut ev_close_skirmish: EventReader<CloseSkirmishMenuEvent>,
    skirmish_ui_query: Query<Entity, With<SkirmishMenuUI>>,
    skirmish_state: Option<Res<SkirmishMenuState>>,
) {
    for _ in ev_close_skirmish.read() {
        if skirmish_state.is_some() {
            // Remove skirmish UI
            for entity in skirmish_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Remove skirmish state
            commands.remove_resource::<SkirmishMenuState>();
        }
    }
}

/// Handle skirmish menu button interactions
fn handle_skirmish_menu_buttons(
    mut button_query: Query<
        (&Interaction, &SkirmishMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_close_skirmish: EventWriter<CloseSkirmishMenuEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    SkirmishMenuButton::QuickMatch => {
                        println!("Quick Match button pressed!");
                        // Quick match functionality would go here
                    }
                    SkirmishMenuButton::CustomGame => {
                        println!("Custom Game button pressed!");
                        // Custom game functionality would go here
                    }
                    SkirmishMenuButton::ChallengeMode => {
                        println!("Challenge Mode button pressed!");
                        // Challenge mode functionality would go here
                    }
                    SkirmishMenuButton::Back => {
                        println!("Back button pressed!");
                        ev_close_skirmish.send(CloseSkirmishMenuEvent);
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.45));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.35));
            }
        }
    }
}

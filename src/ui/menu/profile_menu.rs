use bevy::prelude::*;
use super::components::{MenuUI, create_button, create_title};

/// Plugin for the player profile menu
pub struct ProfileMenuPlugin;

impl Plugin for ProfileMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenProfileMenuEvent>()
           .add_event::<CloseProfileMenuEvent>()
           .add_systems(Update, handle_open_profile_menu_event)
           .add_systems(Update, handle_profile_menu_buttons.run_if(resource_exists::<ProfileMenuState>))
           .add_systems(Update, handle_close_profile_menu_event);
    }
}

/// Event to open the profile menu
#[derive(Event)]
pub struct OpenProfileMenuEvent;

/// Event to close the profile menu
#[derive(Event)]
pub struct CloseProfileMenuEvent;

/// State of the profile menu
#[derive(Resource)]
struct ProfileMenuState;

/// Marker component for profile menu UI elements
#[derive(Component)]
struct ProfileMenuUI;

/// Profile menu button types
#[derive(Component)]
enum ProfileMenuButton {
    PlayerStats,
    Achievements,
    Customization,
    CloudSaves,
    Back,
}

/// Handle the open profile menu event
fn handle_open_profile_menu_event(
    mut commands: Commands,
    mut ev_open_profile: EventReader<OpenProfileMenuEvent>,
    asset_server: Res<AssetServer>,
    profile_state: Option<Res<ProfileMenuState>>,
) {
    for _ in ev_open_profile.read() {
        // Only open profile menu if it's not already open
        if profile_state.is_none() {
            commands.insert_resource(ProfileMenuState);
            
            // Placeholder for profile menu UI
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
                    ProfileMenuUI,
                    MenuUI,
                ))
                .with_children(|parent| {
                    // Profile menu panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(700.0),
                                height: Val::Px(500.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Title
                            create_title(parent, "PLAYER PROFILE", &asset_server, 40.0);
                            
                            // Player info area
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(100.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(20.0)),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Player avatar placeholder
                                    parent.spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Px(80.0),
                                            height: Val::Px(80.0),
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                                        ..default()
                                    });
                                    
                                    // Player name and level
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::Center,
                                                margin: UiRect::all(Val::Px(10.0)),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Commander",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                            
                                            parent.spawn(TextBundle::from_section(
                                                "Level 1 - Recruit",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                                    font_size: 18.0,
                                                    color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                                                },
                                            ));
                                        });
                                });
                            
                            // Option buttons
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        row_gap: Val::Px(15.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Player Statistics button
                                    create_button(parent, "Player Statistics", ProfileMenuButton::PlayerStats, &asset_server, 300.0, 50.0);
                                    
                                    // Achievements button
                                    create_button(parent, "Achievements", ProfileMenuButton::Achievements, &asset_server, 300.0, 50.0);
                                    
                                    // Customization button
                                    create_button(parent, "Customization", ProfileMenuButton::Customization, &asset_server, 300.0, 50.0);
                                    
                                    // Cloud Saves button
                                    create_button(parent, "Cloud Saves", ProfileMenuButton::CloudSaves, &asset_server, 300.0, 50.0);
                                    
                                    // Back button
                                    create_button(parent, "Back", ProfileMenuButton::Back, &asset_server, 300.0, 50.0);
                                });
                        });
                });
        }
    }
}

/// Handle the close profile menu event
fn handle_close_profile_menu_event(
    mut commands: Commands,
    mut ev_close_profile: EventReader<CloseProfileMenuEvent>,
    profile_ui_query: Query<Entity, With<ProfileMenuUI>>,
    profile_state: Option<Res<ProfileMenuState>>,
) {
    for _ in ev_close_profile.read() {
        if profile_state.is_some() {
            // Remove profile UI
            for entity in profile_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Remove profile state
            commands.remove_resource::<ProfileMenuState>();
        }
    }
}

/// Handle profile menu button interactions
fn handle_profile_menu_buttons(
    mut button_query: Query<
        (&Interaction, &ProfileMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_close_profile: EventWriter<CloseProfileMenuEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    ProfileMenuButton::PlayerStats => {
                        println!("Player Statistics button pressed!");
                        // Player statistics functionality would go here
                    }
                    ProfileMenuButton::Achievements => {
                        println!("Achievements button pressed!");
                        // Achievements functionality would go here
                    }
                    ProfileMenuButton::Customization => {
                        println!("Customization button pressed!");
                        // Customization functionality would go here
                    }
                    ProfileMenuButton::CloudSaves => {
                        println!("Cloud Saves button pressed!");
                        // Cloud saves functionality would go here
                    }
                    ProfileMenuButton::Back => {
                        println!("Back button pressed!");
                        ev_close_profile.send(CloseProfileMenuEvent);
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

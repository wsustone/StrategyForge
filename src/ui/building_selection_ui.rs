use bevy::prelude::*;
use crate::components::base_modules::ResourceType;
use crate::components::player::PlayerResources;
use crate::components::unit::Team;
use crate::entities::building_types::BuildingType;
use crate::GameState;

// Component to mark UI elements as part of the building selection UI
#[derive(Component)]
pub struct BuildingSelectionUI;

// Component for the different building options
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BuildingOption {
    pub building_type: BuildingType,
}

// Resource to store the building that is currently being placed
#[derive(Resource)]
pub struct BuildingPlacement {
    pub active: bool,
    pub building_type: Option<BuildingType>,
}

impl Default for BuildingPlacement {
    fn default() -> Self {
        Self {
            active: false,
            building_type: None,
        }
    }
}

// Plugin for the building selection UI
pub struct BuildingSelectionUIPlugin;

impl Plugin for BuildingSelectionUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BuildingPlacement>()
            .add_systems(
                Update,
                (
                    update_building_selection_ui,
                    handle_building_option_interactions,
                    place_building,
                ).run_if(in_state(GameState::Gameplay))
            );
            
        info!("Building Selection UI Plugin initialized");
    }
}

// System to show/hide the building selection UI when Build button is clicked
fn update_building_selection_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<BuildingSelectionUI>>,
    base_action_ui: Query<&crate::ui::base_action_ui::BaseAction, Changed<Interaction>>,
    asset_server: Res<AssetServer>,
) {
    // Check if the Build button was clicked
    let build_button_clicked = base_action_ui
        .iter()
        .any(|action| *action == crate::ui::base_action_ui::BaseAction::Build);
    
    // If Build was clicked and the UI doesn't exist, create it
    if build_button_clicked && ui_query.is_empty() {
        spawn_building_selection_ui(&mut commands, &asset_server);
    }
    // We'll handle closing the UI in the handle_building_option_interactions system
}

// Function to spawn the building selection UI
fn spawn_building_selection_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Create the main container - a panel on the right side of the screen
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    right: Val::Px(20.0),
                    width: Val::Px(300.0),
                    height: Val::Auto,
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Px(10.0),
                    column_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                border_color: BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                ..default()
            },
            BuildingSelectionUI,
            Name::new("Building Selection UI"),
        ))
        .with_children(|parent| {
            // Add title
            parent.spawn(
                TextBundle::from_section(
                    "Select Building",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::srgba(0.95, 0.95, 0.95, 1.0),
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                }),
            );
            
            // Production buildings section
            parent.spawn(
                TextBundle::from_section(
                    "Production Buildings",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                }),
            );
            
            // Production building options
            create_building_option(parent, asset_server, BuildingType::Barracks);
            create_building_option(parent, asset_server, BuildingType::Workshop);
            create_building_option(parent, asset_server, BuildingType::Airfield);
            
            // Resource buildings section
            parent.spawn(
                TextBundle::from_section(
                    "Resource Buildings",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                    },
                ).with_style(Style {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(5.0)),
                    ..default()
                }),
            );
            
            // Resource building options
            create_building_option(parent, asset_server, BuildingType::Sawmill);
            create_building_option(parent, asset_server, BuildingType::StoneMine);
            create_building_option(parent, asset_server, BuildingType::IronMine);
            
            // Special buildings section
            parent.spawn(
                TextBundle::from_section(
                    "Special Buildings",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                    },
                ).with_style(Style {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(5.0)),
                    ..default()
                }),
            );
            
            // Special building options
            create_building_option(parent, asset_server, BuildingType::CommandCenter);
            create_building_option(parent, asset_server, BuildingType::ResearchLab);
            
            // Defense buildings section
            parent.spawn(
                TextBundle::from_section(
                    "Defense Buildings",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                    },
                ).with_style(Style {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(5.0)),
                    ..default()
                }),
            );
            
            // Defense building options
            create_building_option(parent, asset_server, BuildingType::Turret);
            create_building_option(parent, asset_server, BuildingType::AntiAirTurret);
            
            // Cancel button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.7, 0.2, 0.2)),
                    ..default()
                },
                Name::new("Cancel Building Selection"),
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Cancel",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            color: Color::srgba(0.95, 0.95, 0.95, 1.0),
                        },
                    ),
                );
            });
        });

    info!("Building selection UI spawned");
}

// Helper function to create a building option button
fn create_building_option(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    building_type: BuildingType,
) {
    // Get building information
    let (name, wood_cost, stone_cost, iron_cost) = match building_type {
        BuildingType::Barracks => ("Barracks", 80, 50, 0),
        BuildingType::Workshop => ("Workshop", 100, 80, 30),
        BuildingType::Airfield => ("Airfield", 120, 60, 40),
        BuildingType::Sawmill => ("Sawmill", 50, 30, 0),
        BuildingType::StoneMine => ("Stone Mine", 60, 20, 0),
        BuildingType::IronMine => ("Iron Mine", 60, 40, 0),
        BuildingType::CommandCenter => ("Command Center", 200, 150, 100),
        BuildingType::ResearchLab => ("Research Lab", 120, 80, 80),
        BuildingType::Turret => ("Turret", 30, 40, 20),
        BuildingType::AntiAirTurret => ("Anti-Air Turret", 30, 30, 40),
    };
    
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
            BuildingOption {
                building_type,
            },
            Name::new(format!("{} Button", name)),
        ))
        .with_children(|parent| {
            // Left side - building info
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Building name
                parent.spawn(
                    TextBundle::from_section(
                        name,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            color: Color::srgba(0.95, 0.95, 0.95, 1.0),
                        },
                    ),
                );
                
                // Building description (optional)
                // parent.spawn(
                //     TextBundle::from_section(
                //         "Description",
                //         TextStyle {
                //             font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                //             font_size: 12.0,
                //             color: Color::srgb(0.8, 0.8, 0.8),
                //         },
                //     ),
                // );
            });
            
            // Right side - cost information
            parent.spawn(
                TextBundle::from_section(
                    format!("W:{} S:{} I:{}", wood_cost, stone_cost, iron_cost),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 14.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                    },
                ),
            );
        });
}

// System to handle building option interactions
fn handle_building_option_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&BuildingOption>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut building_placement: ResMut<BuildingPlacement>,
    mut ui_query: Query<Entity, With<BuildingSelectionUI>>,
    mut commands: Commands,
) {
    for (interaction, mut color, building_option, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Check if this is a building option or the cancel button
                if let Some(option) = building_option {
                    // Change button color when pressed
                    *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35)); // Green for pressed
                    
                    // Start building placement
                    building_placement.active = true;
                    building_placement.building_type = Some(option.building_type);
                    
                    info!("Selected building: {:?}", option.building_type);
                    
                    // Close the building selection UI
                    for entity in ui_query.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                } else {
                    // This is likely the cancel button
                    // Close the building selection UI without starting placement
                    for entity in ui_query.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                    
                    // Reset building placement
                    building_placement.active = false;
                    building_placement.building_type = None;
                    
                    info!("Canceled building selection");
                }
            }
            Interaction::Hovered => {
                // Change button color when hovered
                *color = BackgroundColor(Color::srgb(0.33, 0.33, 0.33));
            }
            Interaction::None => {
                // Reset button color when not interacting
                if building_option.is_some() {
                    *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                } else {
                    // For the cancel button, keep it red
                    *color = BackgroundColor(Color::srgb(0.7, 0.2, 0.2));
                }
            }
        }
    }
}

// System to handle building placement
fn place_building(
    mut commands: Commands,
    mut building_placement: ResMut<BuildingPlacement>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_resources: Option<ResMut<PlayerResources>>,
) {
    // Only process if building placement is active
    if !building_placement.active || building_placement.building_type.is_none() {
        return;
    }
    
    // Get the building type
    let building_type = building_placement.building_type.unwrap();
    
    // Get the primary window
    let window = windows.single();
    
    // Get the camera transform
    let (camera, camera_transform) = camera_q.single();
    
    // Get cursor position
    if let Some(cursor_position) = window.cursor_position() {
        // Convert screen position to world position
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            // Show a preview of the building at the cursor position
            // (For simplicity, we're not implementing the preview in this example)
            
            // When left mouse button is clicked, place the building
            if mouse_buttons.just_pressed(MouseButton::Left) {
                // Check if player has enough resources
                if let Some(ref mut resources) = player_resources {
                    // Get building costs
                    let (wood_cost, stone_cost, iron_cost) = match building_type {
                        BuildingType::Barracks => (80, 50, 0),
                        BuildingType::Workshop => (100, 80, 30),
                        BuildingType::Airfield => (120, 60, 40),
                        BuildingType::Sawmill => (50, 30, 0),
                        BuildingType::StoneMine => (60, 20, 0),
                        BuildingType::IronMine => (60, 40, 0),
                        BuildingType::CommandCenter => (200, 150, 100),
                        BuildingType::ResearchLab => (120, 80, 80),
                        BuildingType::Turret => (30, 40, 20),
                        BuildingType::AntiAirTurret => (30, 30, 40),
                    };
                    
                    // Find available resources
                    let mut wood_available = 0;
                    let mut stone_available = 0;
                    let mut iron_available = 0;
                    
                    for (res_type, amount) in &resources.resources {
                        match res_type {
                            ResourceType::Wood => wood_available = *amount,
                            ResourceType::Stone => stone_available = *amount,
                            ResourceType::Iron => iron_available = *amount,
                            _ => {} // Ignore other resource types for now
                        }
                    }
                    
                    // Check if player has enough resources
                    if wood_available >= wood_cost && stone_available >= stone_cost && iron_available >= iron_cost {
                        // Deduct resources
                        for (res_type, amount) in &mut resources.resources {
                            match res_type {
                                ResourceType::Wood => *amount -= wood_cost,
                                ResourceType::Stone => *amount -= stone_cost,
                                ResourceType::Iron => *amount -= iron_cost,
                                _ => {} // Ignore other resource types for now
                            }
                        }
                        
                        // Place the building
                        building_type.spawn_building(&mut commands, world_position, Team::Player);
                        info!("Placed {:?} at {:?}", building_type, world_position);
                    } else {
                        info!("Not enough resources to build {:?}", building_type);
                    }
                } else {
                    // No player resources resource, just place the building
                    building_type.spawn_building(&mut commands, world_position, Team::Player);
                    info!("Placed {:?} at {:?} (no resource check)", building_type, world_position);
                }
                
                // End building placement
                building_placement.active = false;
                building_placement.building_type = None;
            }
            // When right mouse button is clicked, cancel building placement
            else if mouse_buttons.just_pressed(MouseButton::Right) {
                building_placement.active = false;
                building_placement.building_type = None;
                info!("Cancelled building placement");
            }
        }
    }
}

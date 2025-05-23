use bevy::prelude::*;
use crate::components::building::{Building, BuildingSpawner};
use crate::components::unit::{Selected, Team};
use crate::entities::building_types::BuildingType;
use crate::GameState;

// Component to mark UI elements as part of the building production UI
#[derive(Component)]
pub struct BuildingProductionUI;

// Component for the different production options
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct ProductionOption {
    pub unit_type: String,
    pub building_entity: Entity,
}

// Plugin for the building production UI
pub struct BuildingProductionUIPlugin;

impl Plugin for BuildingProductionUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_building_production_ui,
                handle_production_button_interactions,
            ).run_if(in_state(GameState::Gameplay)));
            
        info!("Building Production UI Plugin initialized");
    }
}

// System to show/hide the building production UI when a production building is selected/deselected
fn update_building_production_ui(
    mut commands: Commands,
    building_query: Query<(Entity, &BuildingType, &Team, &Building), With<Selected>>,
    ui_query: Query<Entity, With<BuildingProductionUI>>,
    asset_server: Res<AssetServer>,
) {
    // Check if a player's production building is selected
    let mut player_production_building = None;
    
    for (entity, building_type, team, building) in building_query.iter() {
        if *team == Team::Player && building.is_completed {
            match building_type {
                BuildingType::Barracks | BuildingType::Workshop | BuildingType::Airfield => {
                    player_production_building = Some((entity, building_type));
                    break;
                },
                _ => {}
            }
        }
    }
    
    // If a player's production building is selected and the UI doesn't exist, create it
    if let Some((entity, building_type)) = player_production_building {
        if ui_query.is_empty() {
            spawn_building_production_ui(&mut commands, &asset_server, entity, *building_type);
        }
    }
    // If no player's production building is selected but the UI exists, despawn it
    else if !ui_query.is_empty() {
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Function to spawn the building production UI
fn spawn_building_production_ui(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    building_entity: Entity,
    building_type: BuildingType,
) {
    // Create the main container - a panel at the bottom right of the screen
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
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
            BuildingProductionUI,
            Name::new(format!("{:?} Production UI", building_type)),
        ))
        .with_children(|parent| {
            // Add title
            parent.spawn(
                TextBundle::from_section(
                    format!("{:?} Production", building_type),
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
            
            // Add available units based on building type
            match building_type {
                BuildingType::Barracks => {
                    create_production_button(parent, asset_server, "Engineer", building_entity);
                    // Add more units here as they are implemented
                },
                BuildingType::Workshop => {
                    create_production_button(parent, asset_server, "Engineer", building_entity);
                    // Add more units here as they are implemented
                },
                BuildingType::Airfield => {
                    create_production_button(parent, asset_server, "Engineer", building_entity);
                    // Add more units here as they are implemented
                },
                _ => {}
            }
        });

    info!("Building production UI spawned for {:?}", building_type);
}

// Helper function to create a production button
fn create_production_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    unit_type: &str,
    building_entity: Entity,
) {
    // Get cost information based on unit type
    let (cost_wood, cost_stone, cost_iron) = match unit_type {
        "Engineer" => (20, 10, 0),
        // Add more unit types with their costs
        _ => (0, 0, 0),
    };
    
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
            ProductionOption {
                unit_type: unit_type.to_string(),
                building_entity,
            },
            Name::new(format!("Produce {} Button", unit_type)),
        ))
        .with_children(|parent| {
            // Add unit name
            parent.spawn(
                TextBundle::from_section(
                    unit_type,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgba(0.95, 0.95, 0.95, 1.0),
                    },
                ),
            );
            
            // Add cost information
            parent.spawn(
                TextBundle::from_section(
                    format!("W:{} S:{} I:{}", cost_wood, cost_stone, cost_iron),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 14.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 1.0),
                    },
                ),
            );
        });
}

// System to handle production button interactions
fn handle_production_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ProductionOption),
        (Changed<Interaction>, With<Button>),
    >,
    mut building_spawner_query: Query<&mut BuildingSpawner>,
) {
    for (interaction, mut color, production_option) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Change button color when pressed
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35)); // Green for pressed
                
                // Handle the production selection
                if let Ok(mut spawner) = building_spawner_query.get_mut(production_option.building_entity) {
                    // Set the building to produce the selected unit type
                    spawner.unit_type = production_option.unit_type.clone();
                    info!("Set {:?} to produce {}", production_option.building_entity, production_option.unit_type);
                }
            }
            Interaction::Hovered => {
                // Change button color when hovered
                *color = BackgroundColor(Color::srgb(0.33, 0.33, 0.33));
            }
            Interaction::None => {
                // Reset button color when not interacting
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

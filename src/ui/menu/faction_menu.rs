use bevy::prelude::*;
use super::components::{MenuUI, create_button, create_title};

/// Plugin for the faction headquarters menu
pub struct FactionMenuPlugin;

impl Plugin for FactionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenFactionMenuEvent>()
           .add_event::<CloseFactionMenuEvent>()
           .add_systems(Update, handle_open_faction_menu_event)
           .add_systems(Update, handle_faction_menu_buttons.run_if(resource_exists::<FactionMenuState>))
           .add_systems(Update, handle_close_faction_menu_event);
    }
}

/// Event to open the faction menu
#[derive(Event)]
pub struct OpenFactionMenuEvent;

/// Event to close the faction menu
#[derive(Event)]
pub struct CloseFactionMenuEvent;

/// State of the faction menu
#[derive(Resource)]
struct FactionMenuState {
    selected_faction: FactionType,
    first_visible_index: usize,
    cards_per_page: usize,
}

/// Types of factions available
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FactionType {
    Mechanists,
    Synthetics,
    Nomads,
    ArcaneEngineers,
    CorporateMercenaries,
    VoidHarbingers,
    SwarmCollective,
}

/// Faction information for display
struct FactionInfo {
    name: &'static str,
    description: &'static str,
    strengths: [&'static str; 3],
    primary_color: Color,
    secondary_color: Color,
}

/// Get information about a faction
fn get_faction_info(faction_type: FactionType) -> FactionInfo {
    match faction_type {
        FactionType::Mechanists => FactionInfo {
            name: "Mechanists",
            description: "Industrial powerhouse focused on heavy machinery, defensive capabilities, and raw firepower.",
            strengths: ["Heavy Weaponry", "Defensive Structures", "Resource Efficiency"],
            primary_color: Color::srgb(0.6, 0.2, 0.2),
            secondary_color: Color::srgb(0.3, 0.3, 0.3),
        },
        FactionType::Synthetics => FactionInfo {
            name: "Synthetics",
            description: "Advanced AI civilization utilizing energy weapons, computational systems, and adaptive technology.",
            strengths: ["Energy Weapons", "Computational Systems", "Adaptive Technology"],
            primary_color: Color::srgb(0.2, 0.6, 0.8),
            secondary_color: Color::srgb(0.1, 0.2, 0.4),
        },
        FactionType::Nomads => FactionInfo {
            name: "Nomads",
            description: "Desert wanderers with exceptional mobility, scavenging abilities, and survival expertise.",
            strengths: ["Mobility", "Scavenging", "Adaptability"],
            primary_color: Color::srgb(0.8, 0.6, 0.2),
            secondary_color: Color::srgb(0.5, 0.3, 0.1),
        },
        FactionType::ArcaneEngineers => FactionInfo {
            name: "Arcane Engineers",
            description: "Masters of crystal technology and energy field manipulation with unique power systems.",
            strengths: ["Crystal Technology", "Energy Fields", "Power Generation"],
            primary_color: Color::srgb(0.5, 0.2, 0.7),
            secondary_color: Color::srgb(0.2, 0.1, 0.3),
        },
        FactionType::CorporateMercenaries => FactionInfo {
            name: "Corporate Mercenaries",
            description: "Profit-driven specialists with modular technology, economic advantages, and tactical flexibility.",
            strengths: ["Modular Systems", "Economic Bonuses", "Tactical Flexibility"],
            primary_color: Color::srgb(0.2, 0.4, 0.2),
            secondary_color: Color::srgb(0.1, 0.2, 0.1),
        },
        FactionType::VoidHarbingers => FactionInfo {
            name: "Void Harbingers",
            description: "Mysterious manipulators of gravity and dark energy with unique spatial control abilities.",
            strengths: ["Gravity Manipulation", "Dark Energy", "Spatial Control"],
            primary_color: Color::srgb(0.1, 0.1, 0.3),
            secondary_color: Color::srgb(0.05, 0.05, 0.15),
        },
        FactionType::SwarmCollective => FactionInfo {
            name: "Swarm Collective",
            description: "Hive-mind entities with distributed intelligence, rapid reproduction, and overwhelming numbers.",
            strengths: ["Overwhelming Numbers", "Rapid Reproduction", "Distributed Intelligence"],
            primary_color: Color::srgb(0.7, 0.3, 0.0),
            secondary_color: Color::srgb(0.4, 0.2, 0.0),
        },
    }
}

/// Get all faction types
fn get_all_faction_types() -> [FactionType; 7] {
    [
        FactionType::Mechanists,
        FactionType::Synthetics,
        FactionType::Nomads,
        FactionType::ArcaneEngineers,
        FactionType::CorporateMercenaries,
        FactionType::VoidHarbingers,
        FactionType::SwarmCollective,
    ]
}

/// Marker component for faction menu UI elements
#[derive(Component)]
struct FactionMenuUI;

/// Faction menu button types
#[derive(Component)]
enum FactionMenuButton {
    FactionSelect(FactionType),
    PreviousFactions,
    NextFactions,
    TechBrowser,
    UnitEncyclopedia,
    StrategiesAndTactics,
    AchievementsAndUnlocks,
    Back,
}

/// Create a faction card UI element
fn create_faction_card(
    parent: &mut ChildBuilder,
    faction_type: FactionType,
    is_selected: bool,
    asset_server: &Res<AssetServer>,
) {
    let faction_info = get_faction_info(faction_type);
    let border_color = if is_selected {
        Color::srgb(0.9, 0.8, 0.1) // Gold border for selected faction
    } else {
        faction_info.secondary_color
    };
    
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(if is_selected { 3.0 } else { 1.0 })),
                ..default()
            },
            background_color: BackgroundColor(faction_info.primary_color),
            border_color: BorderColor(border_color),
            ..default()
        })
        .with_children(|card| {
            // Faction name header
            card.spawn(TextBundle {
                text: Text::from_section(
                    faction_info.name,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 28.0,
                        color: Color::WHITE,
                    },
                ).with_justify(JustifyText::Center),
                ..default()
            });
            
            // Faction image placeholder (would be replaced with actual faction image)
            card.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(150.0),
                    margin: UiRect::vertical(Val::Px(15.0)),
                    ..default()
                },
                background_color: BackgroundColor(faction_info.secondary_color),
                ..default()
            });
            
            // Faction description
            card.spawn(TextBundle {
                text: Text::from_section(
                    faction_info.description,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ).with_justify(JustifyText::Center),
                style: Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
            
            // Strengths section
            card.spawn(TextBundle {
                text: Text::from_section(
                    "STRENGTHS",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ).with_justify(JustifyText::Center),
                ..default()
            });
            
            // Strengths list
            for strength in faction_info.strengths.iter() {
                card.spawn(TextBundle {
                    text: Text::from_section(
                        format!("• {}", strength),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                    ).with_justify(JustifyText::Center),
                    ..default()
                });
            }
            
            // Select button
            card.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(120.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                },
                background_color: BackgroundColor(faction_info.secondary_color),
                ..default()
            }, FactionMenuButton::FactionSelect(faction_type)))
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Select",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
}

/// Handle the open faction menu event
fn handle_open_faction_menu_event(
    mut commands: Commands,
    mut ev_open_faction: EventReader<OpenFactionMenuEvent>,
    asset_server: Res<AssetServer>,
    faction_state: Option<Res<FactionMenuState>>,
) {
    for _ in ev_open_faction.read() {
        // Only open faction menu if it's not already open
        if faction_state.is_none() {
            commands.insert_resource(FactionMenuState {
                selected_faction: FactionType::Mechanists, // Default to Mechanists
                first_visible_index: 0,
                cards_per_page: 3, // Show 3 faction cards at a time
            });
            
            // Main faction menu UI
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
                    FactionMenuUI,
                    MenuUI,
                ))
                .with_children(|parent| {
                    // Faction menu panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(1000.0),
                                height: Val::Px(650.0),
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
                            create_title(parent, "FACTION HEADQUARTERS", &asset_server, 40.0);
                            
                            // Faction cards carousel
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(420.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceBetween,
                                        margin: UiRect::vertical(Val::Px(20.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|carousel| {
                                    // Previous button
                                    carousel
                                        .spawn((ButtonBundle {
                                            style: Style {
                                                width: Val::Px(50.0),
                                                height: Val::Px(50.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                                            ..default()
                                        }, FactionMenuButton::PreviousFactions))
                                        .with_children(|button| {
                                            button.spawn(TextBundle::from_section(
                                                "<",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                        });
                                    
                                    // Faction cards container
                                    carousel
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                column_gap: Val::Px(20.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|cards_container| {
                                            // Get all faction types and the state
                                            let all_factions = get_all_faction_types();
                                            let first_index = 0; // Starting with first 3 factions
                                            let cards_per_page = 3;
                                            
                                            // Display the first 3 faction cards
                                            for i in 0..cards_per_page {
                                                if i < all_factions.len() {
                                                    let faction_type = all_factions[first_index + i];
                                                    let is_selected = faction_type == FactionType::Mechanists; // Default selected
                                                    create_faction_card(cards_container, faction_type, is_selected, &asset_server);
                                                }
                                            }
                                        });
                                    
                                    // Next button
                                    carousel
                                        .spawn((ButtonBundle {
                                            style: Style {
                                                width: Val::Px(50.0),
                                                height: Val::Px(50.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                                            ..default()
                                        }, FactionMenuButton::NextFactions))
                                        .with_children(|button| {
                                            button.spawn(TextBundle::from_section(
                                                ">",
                                                TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 30.0,
                                                    color: Color::WHITE,
                                                },
                                            ));
                                        });
                                });
                            
                            // Faction HQ options
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(80.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        margin: UiRect::top(Val::Px(20.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Tech Browser button
                                    create_button(parent, "Tech Browser", FactionMenuButton::TechBrowser, &asset_server, 150.0, 60.0);
                                    
                                    // Unit Encyclopedia button
                                    create_button(parent, "Unit Encyclopedia", FactionMenuButton::UnitEncyclopedia, &asset_server, 150.0, 60.0);
                                    
                                    // Strategies & Tactics button
                                    create_button(parent, "Strategies & Tactics", FactionMenuButton::StrategiesAndTactics, &asset_server, 150.0, 60.0);
                                    
                                    // Achievements & Unlocks button
                                    create_button(parent, "Achievements & Unlocks", FactionMenuButton::AchievementsAndUnlocks, &asset_server, 150.0, 60.0);
                                });
                            
                            // Back button
                            create_button(parent, "Back to Main Menu", FactionMenuButton::Back, &asset_server, 200.0, 50.0);
                        });
                });
        }
    }
}

/// Handle the close faction menu event
fn handle_close_faction_menu_event(
    mut commands: Commands,
    mut ev_close_faction: EventReader<CloseFactionMenuEvent>,
    faction_ui_query: Query<Entity, With<FactionMenuUI>>,
    faction_state: Option<Res<FactionMenuState>>,
) {
    for _ in ev_close_faction.read() {
        if faction_state.is_some() {
            // Remove faction UI
            for entity in faction_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Remove faction state
            commands.remove_resource::<FactionMenuState>();
        }
    }
}

/// Handle faction menu button interactions
fn handle_faction_menu_buttons(
    _commands: Commands,
    mut button_query: Query<
        (&Interaction, &FactionMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut faction_state: ResMut<FactionMenuState>,
    mut ev_close_faction: EventWriter<CloseFactionMenuEvent>,
    _asset_server: Res<AssetServer>,
    _faction_cards_query: Query<Entity, With<FactionMenuUI>>,
) {
    let all_factions = get_all_faction_types();
    let total_factions = all_factions.len();
    
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    FactionMenuButton::FactionSelect(faction_type) => {
                        faction_state.selected_faction = *faction_type;
                        println!("Selected faction: {:?}", faction_type);
                        
                        // Update UI to reflect the selection
                        // In a full implementation, we would update the UI to show the selected faction
                    }
                    FactionMenuButton::PreviousFactions => {
                        // Navigate to previous set of factions
                        if faction_state.first_visible_index > 0 {
                            faction_state.first_visible_index -= 1;
                            println!("Showing previous faction set, starting at index {}", faction_state.first_visible_index);
                            
                            // In a full implementation, we would update the UI to show the new set of factions
                            // This would involve despawning the current cards and spawning new ones
                        }
                    }
                    FactionMenuButton::NextFactions => {
                        // Navigate to next set of factions
                        if faction_state.first_visible_index + faction_state.cards_per_page < total_factions {
                            faction_state.first_visible_index += 1;
                            println!("Showing next faction set, starting at index {}", faction_state.first_visible_index);
                            
                            // In a full implementation, we would update the UI to show the new set of factions
                            // This would involve despawning the current cards and spawning new ones
                        }
                    }
                    FactionMenuButton::TechBrowser => {
                        println!("Opening Tech Browser for {:?}", faction_state.selected_faction);
                        // Tech browser functionality would go here
                    }
                    FactionMenuButton::UnitEncyclopedia => {
                        println!("Opening Unit Encyclopedia for {:?}", faction_state.selected_faction);
                        // Unit encyclopedia functionality would go here
                    }
                    FactionMenuButton::StrategiesAndTactics => {
                        println!("Opening Strategies & Tactics for {:?}", faction_state.selected_faction);
                        // Strategies & Tactics functionality would go here
                    }
                    FactionMenuButton::AchievementsAndUnlocks => {
                        println!("Opening Achievements & Unlocks for {:?}", faction_state.selected_faction);
                        // Achievements & Unlocks functionality would go here
                    }
                    FactionMenuButton::Back => {
                        ev_close_faction.send(CloseFactionMenuEvent);
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

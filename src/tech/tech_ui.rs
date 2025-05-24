//! Technology UI implementation
//!
//! This module handles the user interface for the technology tree,
//! including tech tree visualization, research buttons, and progress indicators.

use bevy::prelude::*;
use super::tech_tree::{TechTree, TechNode, TechCategory, TechStatus};
use super::faction_tech::FactionTechTrees;
use super::tech_requirements::{can_afford_technology, pay_research_cost};
use crate::resources::player_resources::PlayerResources;

/// Plugin for technology UI systems
pub struct TechUIPlugin;

impl Plugin for TechUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_tech_ui)
            .add_systems(Update, handle_tech_ui_interaction);
    }
}

/// Component to mark tech tree UI elements
#[derive(Component)]
pub struct TechTreeUI;

/// Component to mark a tech node UI element
#[derive(Component)]
pub struct TechNodeUI {
    pub tech_id: String,
    pub faction: String,
}

/// Component for tech category tabs
#[derive(Component)]
pub struct TechCategoryTab {
    pub category: TechCategory,
}

/// System to update the tech UI based on current research status
fn update_tech_ui(
    tech_trees: Res<FactionTechTrees>,
    mut query: Query<(&mut Text, &TechNodeUI)>,
    mut progress_bars: Query<(&mut Style, &TechNodeUI), Without<Text>>,
) {
    // Update text and progress bars for each tech node in the UI
    for (mut text, tech_ui) in query.iter_mut() {
        if let Some(tree) = tech_trees.trees.get(&tech_ui.faction) {
            if let Some(tech) = tree.get_technology(&tech_ui.tech_id) {
                // Update text based on tech status
                let status_text = match tech.status {
                    TechStatus::Locked => "Locked",
                    TechStatus::Available => "Available",
                    TechStatus::Researching => "Researching...",
                    TechStatus::Researched => "Researched",
                };
                
                text.sections[0].value = format!("{}\n{}", tech.name, status_text);
            }
        }
    }
    
    // Update progress bars
    for (mut style, tech_ui) in progress_bars.iter_mut() {
        if let Some(tree) = tech_trees.trees.get(&tech_ui.faction) {
            if let Some(tech) = tree.get_technology(&tech_ui.tech_id) {
                // Update progress bar width based on research progress
                if tech.status == TechStatus::Researching {
                    style.width = Val::Percent(tech.research_progress * 100.0);
                } else if tech.status == TechStatus::Researched {
                    style.width = Val::Percent(100.0);
                } else {
                    style.width = Val::Percent(0.0);
                }
            }
        }
    }
}

/// System to handle interactions with the tech UI
fn handle_tech_ui_interaction(
    mut interaction_query: Query<
        (&Interaction, &TechNodeUI),
        (Changed<Interaction>, With<Button>)
    >,
    mut tech_trees: ResMut<FactionTechTrees>,
    mut player_resources: ResMut<PlayerResources>,
) {
    for (interaction, tech_ui) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(tree) = tech_trees.trees.get_mut(&tech_ui.faction) {
                if let Some(tech) = tree.get_technology(&tech_ui.tech_id) {
                    // Clone the tech to avoid borrowing issues
                    let tech_clone = tech.clone();
                    
                    // Check if the tech is available and can be researched
                    if tech_clone.status == TechStatus::Available {
                        // Check if player can afford the research
                        if can_afford_technology(&tech_clone, &player_resources) {
                            // Pay the research cost
                            if pay_research_cost(&tech_clone, &mut player_resources) {
                                // Start researching the technology
                                tree.start_research(&tech_ui.tech_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Spawn the tech tree UI for a faction
pub fn spawn_tech_tree_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    faction: &str,
    tech_tree: &TechTree,
) {
    // This would create the actual UI elements for the tech tree
    // Including nodes, connections, category tabs, etc.
    
    // Root tech tree UI entity
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(TechTreeUI)
        .with_children(|parent| {
            // Category tabs
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tabs| {
                    // Military tab
                    spawn_category_tab(tabs, asset_server, TechCategory::Military, "Military");
                    
                    // Economy tab
                    spawn_category_tab(tabs, asset_server, TechCategory::Economy, "Economy");
                    
                    // Infrastructure tab
                    spawn_category_tab(tabs, asset_server, TechCategory::Infrastructure, "Infrastructure");
                    
                    // Special tab
                    spawn_category_tab(tabs, asset_server, TechCategory::Special, "Special");
                });
            
            // Tech tree content area
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|content| {
                    // Spawn tech nodes for each category
                    // This is a simplified version - a real implementation would position nodes
                    // based on their relationships and create connecting lines
                    
                    // Military technologies
                    let military_techs = tech_tree.get_technologies_by_category(TechCategory::Military);
                    for tech in military_techs {
                        spawn_tech_node(content, asset_server, tech, faction);
                    }
                    
                    // Economy technologies
                    let economy_techs = tech_tree.get_technologies_by_category(TechCategory::Economy);
                    for tech in economy_techs {
                        spawn_tech_node(content, asset_server, tech, faction);
                    }
                    
                    // Infrastructure technologies
                    let infra_techs = tech_tree.get_technologies_by_category(TechCategory::Infrastructure);
                    for tech in infra_techs {
                        spawn_tech_node(content, asset_server, tech, faction);
                    }
                    
                    // Special technologies
                    let special_techs = tech_tree.get_technologies_by_category(TechCategory::Special);
                    for tech in special_techs {
                        spawn_tech_node(content, asset_server, tech, faction);
                    }
                });
        });
}

/// Spawn a category tab button
fn spawn_category_tab(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    category: TechCategory,
    label: &str,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(TechCategoryTab { category })
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        });
}

/// Spawn a tech node button
fn spawn_tech_node(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    tech: &TechNode,
    faction: &str,
) {
    // Node background color based on status
    let bg_color = match tech.status {
        TechStatus::Locked => Color::rgb(0.2, 0.2, 0.2),
        TechStatus::Available => Color::rgb(0.0, 0.5, 0.0),
        TechStatus::Researching => Color::rgb(0.0, 0.0, 0.8),
        TechStatus::Researched => Color::rgb(0.8, 0.8, 0.0),
    };
    
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(100.0),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: bg_color.into(),
            ..default()
        })
        .insert(TechNodeUI {
            tech_id: tech.id.clone(),
            faction: faction.to_string(),
        })
        .with_children(|button| {
            // Tech icon (if available)
            if let Some(icon_path) = &tech.icon {
                button.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load(icon_path)),
                    ..default()
                });
            }
            
            // Tech name and status
            let status_text = match tech.status {
                TechStatus::Locked => "Locked",
                TechStatus::Available => "Available",
                TechStatus::Researching => "Researching...",
                TechStatus::Researched => "Researched",
            };
            
            button.spawn(TextBundle::from_section(
                format!("{}\n{}", tech.name, status_text),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ));
            
            // Progress bar (for researching technologies)
            if tech.status == TechStatus::Researching || tech.status == TechStatus::Researched {
                button
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(5.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .with_children(|progress_container| {
                        // Progress fill
                        progress_container.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(tech.research_progress * 100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(0.0, 0.8, 0.0).into(),
                            ..default()
                        })
                        .insert(TechNodeUI {
                            tech_id: tech.id.clone(),
                            faction: faction.to_string(),
                        });
                    });
            }
        });
}

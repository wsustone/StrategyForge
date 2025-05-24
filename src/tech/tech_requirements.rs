//! Technology requirements system
//!
//! This module handles the requirements for researching technologies,
//! including prerequisite technologies, resource costs, and building requirements.

use bevy::prelude::*;
use super::tech_tree::{TechTree, TechNode, TechStatus};
use super::faction_tech::FactionTechTrees;
use crate::resources::player_resources::PlayerResources;

/// Plugin for technology requirements systems
pub struct TechRequirementPlugin;

impl Plugin for TechRequirementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, check_tech_availability);
    }
}

/// System to check and update technology availability based on prerequisites
fn check_tech_availability(
    mut tech_trees: ResMut<FactionTechTrees>,
    player_resources: Res<PlayerResources>,
) {
    for tree in tech_trees.trees.values_mut() {
        update_tech_availability(tree);
    }
}

/// Update the availability status of technologies in a tech tree
fn update_tech_availability(tech_tree: &mut TechTree) {
    // Create a list of technologies to update
    let tech_ids: Vec<String> = tech_tree.technologies.keys().cloned().collect();
    
    for tech_id in tech_ids {
        if let Some(tech) = tech_tree.get_technology_mut(&tech_id) {
            // Skip technologies that are already researched or researching
            if tech.status == TechStatus::Researched || tech.status == TechStatus::Researching {
                continue;
            }
            
            // Check if all prerequisites are met
            let mut all_prereqs_met = true;
            for prereq_id in &tech.prerequisites {
                if let Some(prereq) = tech_tree.get_technology(prereq_id) {
                    if prereq.status != TechStatus::Researched {
                        all_prereqs_met = false;
                        break;
                    }
                } else {
                    all_prereqs_met = false;
                    break;
                }
            }
            
            // Update status based on prerequisites
            if all_prereqs_met {
                tech.status = TechStatus::Available;
            } else {
                tech.status = TechStatus::Locked;
            }
        }
    }
}

/// Check if a player can afford to research a technology
pub fn can_afford_technology(
    tech: &TechNode,
    player_resources: &PlayerResources,
) -> bool {
    for (resource_name, cost) in &tech.research_cost {
        // This would need to be adapted to your actual resource system
        if let Some(current_amount) = player_resources.get_resource(resource_name) {
            if current_amount < *cost {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

/// Pay the cost to research a technology
pub fn pay_research_cost(
    tech: &TechNode,
    player_resources: &mut PlayerResources,
) -> bool {
    // First check if the player can afford it
    if !can_afford_technology(tech, player_resources) {
        return false;
    }
    
    // Deduct the resources
    for (resource_name, cost) in &tech.research_cost {
        player_resources.spend_resource(resource_name, *cost);
    }
    
    true
}

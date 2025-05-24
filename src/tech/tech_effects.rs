//! Technology effects implementation
//!
//! This module handles the application of technology effects to game entities,
//! such as stat bonuses, new abilities, and unlocked units/buildings.

use bevy::prelude::*;
use super::tech_tree::{TechTree, TechNode, TechStatus};
use super::faction_tech::FactionTechTrees;

/// Plugin for technology effects systems
pub struct TechEffectPlugin;

impl Plugin for TechEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, apply_tech_effects);
    }
}

/// System to apply technology effects to entities
fn apply_tech_effects(
    tech_trees: Res<FactionTechTrees>,
    // Add queries for entities that can be affected by technologies
) {
    // Implementation will apply effects of researched technologies to appropriate entities
    // For example, applying damage bonuses, movement speed increases, etc.
}

/// Apply effects of a specific technology
pub fn apply_technology_effect(tech_id: &str, tech_trees: &FactionTechTrees, faction: &str) {
    if let Some(tree) = tech_trees.trees.get(faction) {
        if let Some(tech) = tree.get_technology(tech_id) {
            if tech.status == TechStatus::Researched {
                // Apply the effect based on the technology ID
                match tech_id {
                    "basic_ballistics" => {
                        // Apply +10% projectile accuracy
                        // This would be implemented with actual game mechanics
                    },
                    "steam_power" => {
                        // Apply +5% movement speed to mechanical units
                    },
                    // Add cases for other technologies
                    _ => {
                        // Unknown technology
                    }
                }
            }
        }
    }
}

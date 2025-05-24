//! Technology tree system for StrategyForge
//! 
//! This module implements a technology tree system that allows different factions
//! to research and unlock new technologies, units, buildings, and abilities.

mod tech_tree;
mod faction_tech;
mod tech_effects;
mod tech_requirements;
mod tech_ui;

pub use tech_tree::{TechTree, TechNode, TechCategory, TechLevel, TechStatus};
pub use faction_tech::{FactionTech, FactionTechPlugin};
pub use tech_effects::TechEffectPlugin;
pub use tech_requirements::TechRequirementPlugin;
pub use tech_ui::TechUIPlugin;

use bevy::prelude::*;

/// Main plugin for the technology system
pub struct TechPlugin;

impl Plugin for TechPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FactionTechPlugin)
            .add_plugins(TechEffectPlugin)
            .add_plugins(TechRequirementPlugin)
            .add_plugins(TechUIPlugin)
            .register_type::<TechTree>()
            .register_type::<TechNode>()
            .register_type::<TechCategory>()
            .register_type::<TechLevel>()
            .register_type::<TechStatus>();
    }
}

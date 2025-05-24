//! Faction-specific technology implementations
//!
//! This module contains technology trees for different factions, each with their
//! own unique technologies, strengths, and progression paths.

use bevy::prelude::*;
use std::collections::HashMap;
use crate::resources::ResourceType;
use super::tech_tree::{TechTree, TechNode, TechCategory, TechLevel, TechStatus};

/// Plugin for faction-specific technology systems
pub struct FactionTechPlugin;

impl Plugin for FactionTechPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, initialize_faction_tech_trees)
            .add_systems(Update, update_research_progress);
    }
}

/// Component to mark an entity as having faction tech
#[derive(Component)]
pub struct FactionTech;

/// Resource to store all faction tech trees
#[derive(Resource)]
pub struct FactionTechTrees {
    pub trees: HashMap<String, TechTree>,
}

impl Default for FactionTechTrees {
    fn default() -> Self {
        Self {
            trees: HashMap::new(),
        }
    }
}

/// Initialize all faction tech trees
fn initialize_faction_tech_trees(mut commands: Commands) {
    let mut tech_trees = FactionTechTrees::default();
    
    // Add faction tech trees
    tech_trees.trees.insert("mechanists".to_string(), create_mechanist_tech_tree());
    tech_trees.trees.insert("naturalists".to_string(), create_naturalist_tech_tree());
    tech_trees.trees.insert("synthetics".to_string(), create_synthetic_tech_tree());
    
    commands.insert_resource(tech_trees);
}

/// Update research progress for all faction tech trees
fn update_research_progress(
    mut tech_trees: ResMut<FactionTechTrees>,
    time: Res<Time>,
) {
    for tree in tech_trees.trees.values_mut() {
        tree.update_research(time.delta_seconds());
    }
}

/// Create the Mechanist faction tech tree
///
/// The Mechanists are a faction focused on heavy machinery, industrial production,
/// and powerful but slow-moving units. They excel at defense and resource production.
fn create_mechanist_tech_tree() -> TechTree {
    let mut tree = TechTree::new("Mechanists");
    
    // Military Technologies - Tier 1 (Basic)
    let basic_ballistics = TechNode {
        id: "basic_ballistics".to_string(),
        name: "Basic Ballistics".to_string(),
        description: "Unlocks basic kinetic weapons and improves projectile accuracy by 10%".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 50.0);
            cost
        },
        research_time: 60.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["advanced_ballistics".to_string(), "heavy_armor".to_string()],
        icon: Some("icons/tech/basic_ballistics.png".to_string()),
    };
    
    let steam_power = TechNode {
        id: "steam_power".to_string(),
        name: "Steam Power".to_string(),
        description: "Unlocks steam-powered units and buildings. Increases base movement speed by 5%".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 30.0);
            cost.insert("wood".to_string(), 40.0);
            cost
        },
        research_time: 45.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["combustion_engine".to_string(), "steam_turbines".to_string()],
        icon: Some("icons/tech/steam_power.png".to_string()),
    };
    
    // Military Technologies - Tier 2 (Advanced)
    let advanced_ballistics = TechNode {
        id: "advanced_ballistics".to_string(),
        name: "Advanced Ballistics".to_string(),
        description: "Improves kinetic weapon damage by 20% and range by 15%".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Advanced,
        status: TechStatus::Locked,
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 100.0);
            cost.insert("crystal".to_string(), 50.0);
            cost
        },
        research_time: 120.0,
        research_progress: 0.0,
        prerequisites: vec!["basic_ballistics".to_string()],
        unlocks: vec!["artillery_systems".to_string()],
        icon: Some("icons/tech/advanced_ballistics.png".to_string()),
    };
    
    let heavy_armor = TechNode {
        id: "heavy_armor".to_string(),
        name: "Heavy Armor Plating".to_string(),
        description: "Increases unit and building armor by 25% but reduces movement speed by 10%".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Advanced,
        status: TechStatus::Locked,
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 150.0);
            cost.insert("stone".to_string(), 75.0);
            cost
        },
        research_time: 90.0,
        research_progress: 0.0,
        prerequisites: vec!["basic_ballistics".to_string()],
        unlocks: vec!["composite_armor".to_string()],
        icon: Some("icons/tech/heavy_armor.png".to_string()),
    };
    
    // Economy Technologies - Tier 1 (Basic)
    let industrial_mining = TechNode {
        id: "industrial_mining".to_string(),
        name: "Industrial Mining".to_string(),
        description: "Increases resource gathering speed by 15% for all mineral resources".to_string(),
        category: TechCategory::Economy,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 40.0);
            cost.insert("wood".to_string(), 30.0);
            cost
        },
        research_time: 50.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["automated_mining".to_string(), "refining_processes".to_string()],
        icon: Some("icons/tech/industrial_mining.png".to_string()),
    };
    
    // Add more Mechanist technologies...
    
    // Add all technologies to the tree
    tree.add_technology(basic_ballistics);
    tree.add_technology(steam_power);
    tree.add_technology(advanced_ballistics);
    tree.add_technology(heavy_armor);
    tree.add_technology(industrial_mining);
    
    tree
}

/// Create the Naturalist faction tech tree
///
/// The Naturalists are a faction that harnesses natural forces and biological engineering.
/// They excel at mobility, regeneration, and adaptability to different environments.
fn create_naturalist_tech_tree() -> TechTree {
    let mut tree = TechTree::new("Naturalists");
    
    // Military Technologies - Tier 1 (Basic)
    let organic_compounds = TechNode {
        id: "organic_compounds".to_string(),
        name: "Organic Compounds".to_string(),
        description: "Unlocks basic bio-weapons and increases unit health regeneration by 5%".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("wood".to_string(), 50.0);
            cost.insert("crystal".to_string(), 20.0);
            cost
        },
        research_time: 60.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["advanced_biochemistry".to_string(), "natural_camouflage".to_string()],
        icon: Some("icons/tech/organic_compounds.png".to_string()),
    };
    
    let natural_mobility = TechNode {
        id: "natural_mobility".to_string(),
        name: "Natural Mobility".to_string(),
        description: "Increases unit movement speed by 15% and allows traversal of difficult terrain".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("wood".to_string(), 40.0);
            cost.insert("crystal".to_string(), 15.0);
            cost
        },
        research_time: 45.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["enhanced_mobility".to_string(), "terrain_adaptation".to_string()],
        icon: Some("icons/tech/natural_mobility.png".to_string()),
    };
    
    // Military Technologies - Tier 2 (Advanced)
    let advanced_biochemistry = TechNode {
        id: "advanced_biochemistry".to_string(),
        name: "Advanced Biochemistry".to_string(),
        description: "Improves bio-weapon damage by 20% and adds poison effects to attacks".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Advanced,
        status: TechStatus::Locked,
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("wood".to_string(), 80.0);
            cost.insert("crystal".to_string(), 60.0);
            cost
        },
        research_time: 120.0,
        research_progress: 0.0,
        prerequisites: vec!["organic_compounds".to_string()],
        unlocks: vec!["biological_warfare".to_string()],
        icon: Some("icons/tech/advanced_biochemistry.png".to_string()),
    };
    
    // Economy Technologies - Tier 1 (Basic)
    let sustainable_harvesting = TechNode {
        id: "sustainable_harvesting".to_string(),
        name: "Sustainable Harvesting".to_string(),
        description: "Resources regenerate 20% faster and harvesting is 10% more efficient".to_string(),
        category: TechCategory::Economy,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("wood".to_string(), 50.0);
            cost.insert("crystal".to_string(), 20.0);
            cost
        },
        research_time: 50.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["symbiotic_gathering".to_string(), "natural_refinement".to_string()],
        icon: Some("icons/tech/sustainable_harvesting.png".to_string()),
    };
    
    // Add more Naturalist technologies...
    
    // Add all technologies to the tree
    tree.add_technology(organic_compounds);
    tree.add_technology(natural_mobility);
    tree.add_technology(advanced_biochemistry);
    tree.add_technology(sustainable_harvesting);
    
    tree
}

/// Create the Synthetic faction tech tree
///
/// The Synthetics are a faction of advanced AI and robotic entities.
/// They excel at technology, efficiency, and adaptation, but require
/// more energy to operate their advanced systems.
fn create_synthetic_tech_tree() -> TechTree {
    let mut tree = TechTree::new("Synthetics");
    
    // Military Technologies - Tier 1 (Basic)
    let energy_weapons = TechNode {
        id: "energy_weapons".to_string(),
        name: "Energy Weapons".to_string(),
        description: "Unlocks basic energy weapons with 15% increased damage but 20% higher energy cost".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("crystal".to_string(), 50.0);
            cost.insert("energy".to_string(), 100.0);
            cost
        },
        research_time: 60.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["advanced_energy_weapons".to_string(), "shield_technology".to_string()],
        icon: Some("icons/tech/energy_weapons.png".to_string()),
    };
    
    let drone_swarms = TechNode {
        id: "drone_swarms".to_string(),
        name: "Drone Swarms".to_string(),
        description: "Unlocks drone units that can be produced quickly and in large numbers".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 30.0);
            cost.insert("energy".to_string(), 80.0);
            cost
        },
        research_time: 45.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["advanced_drones".to_string(), "swarm_intelligence".to_string()],
        icon: Some("icons/tech/drone_swarms.png".to_string()),
    };
    
    // Military Technologies - Tier 2 (Advanced)
    let advanced_energy_weapons = TechNode {
        id: "advanced_energy_weapons".to_string(),
        name: "Advanced Energy Weapons".to_string(),
        description: "Improves energy weapon damage by 25% and reduces energy cost by 10%".to_string(),
        category: TechCategory::Military,
        level: TechLevel::Advanced,
        status: TechStatus::Locked,
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("crystal".to_string(), 100.0);
            cost.insert("energy".to_string(), 200.0);
            cost
        },
        research_time: 120.0,
        research_progress: 0.0,
        prerequisites: vec!["energy_weapons".to_string()],
        unlocks: vec!["particle_beam_technology".to_string()],
        icon: Some("icons/tech/advanced_energy_weapons.png".to_string()),
    };
    
    // Economy Technologies - Tier 1 (Basic)
    let automated_extraction = TechNode {
        id: "automated_extraction".to_string(),
        name: "Automated Extraction".to_string(),
        description: "Resource gathering is 25% more efficient but requires 15% more energy".to_string(),
        category: TechCategory::Economy,
        level: TechLevel::Basic,
        status: TechStatus::Available, // Available from the start
        research_cost: {
            let mut cost = HashMap::new();
            cost.insert("iron".to_string(), 40.0);
            cost.insert("energy".to_string(), 80.0);
            cost
        },
        research_time: 50.0,
        research_progress: 0.0,
        prerequisites: Vec::new(),
        unlocks: vec!["nanite_harvesting".to_string(), "matter_conversion".to_string()],
        icon: Some("icons/tech/automated_extraction.png".to_string()),
    };
    
    // Add more Synthetic technologies...
    
    // Add all technologies to the tree
    tree.add_technology(energy_weapons);
    tree.add_technology(drone_swarms);
    tree.add_technology(advanced_energy_weapons);
    tree.add_technology(automated_extraction);
    
    tree
}

/// Get a faction's tech tree by name
pub fn get_faction_tech_tree(tech_trees: &FactionTechTrees, faction_name: &str) -> Option<&TechTree> {
    tech_trees.trees.get(faction_name)
}

/// Get a mutable reference to a faction's tech tree by name
pub fn get_faction_tech_tree_mut(tech_trees: &mut FactionTechTrees, faction_name: &str) -> Option<&mut TechTree> {
    tech_trees.trees.get_mut(faction_name)
}

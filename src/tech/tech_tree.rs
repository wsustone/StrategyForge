//! Technology tree definitions and implementations
//!
//! This module contains the core structures for technology trees, including
//! tech nodes, categories, levels, and status tracking.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use std::collections::HashMap;

/// Technology categories to organize tech trees
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub enum TechCategory {
    /// Military technologies for combat units and weapons
    Military,
    /// Economic technologies for resource gathering and production
    Economy,
    /// Infrastructure technologies for base building and expansion
    Infrastructure,
    /// Special technologies unique to each faction
    Special,
}

/// Technology levels representing progression tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Component)]
pub enum TechLevel {
    Basic,
    Advanced,
    Experimental,
    Ultimate,
}

/// Current status of a technology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub enum TechStatus {
    /// Not yet available for research
    Locked,
    /// Available for research but not yet researched
    Available,
    /// Currently being researched
    Researching,
    /// Research completed
    Researched,
}

/// Represents a single technology node in the tech tree
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TechNode {
    /// Unique identifier for the technology
    pub id: String,
    /// Display name of the technology
    pub name: String,
    /// Description of the technology and its effects
    pub description: String,
    /// Category of the technology
    pub category: TechCategory,
    /// Level/tier of the technology
    pub level: TechLevel,
    /// Current research status
    pub status: TechStatus,
    /// Research cost in resources
    pub research_cost: HashMap<String, f32>,
    /// Time required to research (in seconds)
    pub research_time: f32,
    /// Progress of research (0.0 - 1.0)
    pub research_progress: f32,
    /// IDs of technologies that must be researched first
    pub prerequisites: Vec<String>,
    /// IDs of technologies that this unlocks
    pub unlocks: Vec<String>,
    /// Icon or sprite to represent this technology
    pub icon: Option<String>,
}

impl Default for TechNode {
    fn default() -> Self {
        Self {
            id: "default_tech".to_string(),
            name: "Default Technology".to_string(),
            description: "A placeholder technology".to_string(),
            category: TechCategory::Military,
            level: TechLevel::Basic,
            status: TechStatus::Locked,
            research_cost: HashMap::new(),
            research_time: 60.0,
            research_progress: 0.0,
            prerequisites: Vec::new(),
            unlocks: Vec::new(),
            icon: None,
        }
    }
}

/// Represents a complete technology tree for a faction
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TechTree {
    /// Name of the faction this tech tree belongs to
    pub faction_name: String,
    /// All technology nodes in this tree
    pub technologies: HashMap<String, TechNode>,
    /// Currently researching technology (if any)
    pub current_research: Option<String>,
    /// Research points accumulated
    pub research_points: f32,
    /// Research rate (points per second)
    pub research_rate: f32,
}

impl Default for TechTree {
    fn default() -> Self {
        Self {
            faction_name: "Default Faction".to_string(),
            technologies: HashMap::new(),
            current_research: None,
            research_points: 0.0,
            research_rate: 1.0,
        }
    }
}

impl TechTree {
    /// Create a new tech tree for a faction
    pub fn new(faction_name: &str) -> Self {
        Self {
            faction_name: faction_name.to_string(),
            ..Default::default()
        }
    }

    /// Add a technology to the tree
    pub fn add_technology(&mut self, tech: TechNode) {
        self.technologies.insert(tech.id.clone(), tech);
    }

    /// Get a technology by its ID
    pub fn get_technology(&self, id: &str) -> Option<&TechNode> {
        self.technologies.get(id)
    }

    /// Get a mutable reference to a technology by its ID
    pub fn get_technology_mut(&mut self, id: &str) -> Option<&mut TechNode> {
        self.technologies.get_mut(id)
    }

    /// Start researching a technology
    pub fn start_research(&mut self, tech_id: &str) -> bool {
        if let Some(tech) = self.get_technology_mut(tech_id) {
            if tech.status == TechStatus::Available {
                tech.status = TechStatus::Researching;
                self.current_research = Some(tech_id.to_string());
                return true;
            }
        }
        false
    }

    /// Update research progress
    pub fn update_research(&mut self, delta_time: f32) {
        if let Some(tech_id) = &self.current_research {
            if let Some(tech) = self.get_technology_mut(tech_id) {
                tech.research_progress += (self.research_rate * delta_time) / tech.research_time;
                
                if tech.research_progress >= 1.0 {
                    tech.research_progress = 1.0;
                    tech.status = TechStatus::Researched;
                    self.current_research = None;
                    
                    // Unlock technologies that depend on this one
                    for unlock_id in &tech.unlocks.clone() {
                        if let Some(unlock_tech) = self.get_technology_mut(unlock_id) {
                            // Check if all prerequisites are met
                            let mut all_prereqs_met = true;
                            for prereq_id in &unlock_tech.prerequisites {
                                if let Some(prereq) = self.get_technology(prereq_id) {
                                    if prereq.status != TechStatus::Researched {
                                        all_prereqs_met = false;
                                        break;
                                    }
                                } else {
                                    all_prereqs_met = false;
                                    break;
                                }
                            }
                            
                            if all_prereqs_met {
                                unlock_tech.status = TechStatus::Available;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Check if a technology is researched
    pub fn is_researched(&self, tech_id: &str) -> bool {
        if let Some(tech) = self.get_technology(tech_id) {
            return tech.status == TechStatus::Researched;
        }
        false
    }

    /// Get all technologies in a specific category
    pub fn get_technologies_by_category(&self, category: TechCategory) -> Vec<&TechNode> {
        self.technologies
            .values()
            .filter(|tech| tech.category == category)
            .collect()
    }

    /// Get all technologies at a specific level
    pub fn get_technologies_by_level(&self, level: TechLevel) -> Vec<&TechNode> {
        self.technologies
            .values()
            .filter(|tech| tech.level == level)
            .collect()
    }
}

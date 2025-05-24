//! StrategyForge - A real-time strategy game built with Bevy

// Re-export the main crate dependencies
pub use bevy;
use bevy::prelude::*;

// Public modules
pub mod components;
pub mod entities;
pub mod resources;
pub mod sprites;
pub mod states;
pub mod systems;
pub mod ui;
pub mod units;
pub mod utils;

// Re-export commonly used items
pub mod prelude {
    // Re-export specific items from each module to avoid ambiguity
    pub use crate::components::{
        unit::{Unit, Team, UnitState, Selectable, Selected},
        building::Building,
        resource::ResourceNode,
        strategic::StrategicLocation,
        base_modules::ResourceType,
        player::PlayerResources,
    };
    
    pub use crate::resources::*;
    pub use crate::sprites::*;
    pub use crate::states::*;
    pub use crate::states::game_state::GameState;
    pub use crate::systems::*;
    pub use crate::utils::*;
    
    // Re-export bevy prelude for convenience
    pub use bevy::prelude::*;
}

/// Plugin for the game
pub struct StrategyForgePlugin;

impl Plugin for StrategyForgePlugin {
    fn build(&self, app: &mut App) {
        // Add game state management
        app.add_plugins(states::GameStatePlugin);
        
        // Add sprite loading
        app.add_plugins(sprites::SpriteLoaderPlugin);
        
        // Add other game systems and resources here
        app.add_systems(Startup, setup);
        
        // Add unit plugins
        app.add_plugins(units::EngineerPlugin);
    }
}

/// Setup system for the game
fn setup() {
    // Initial game setup
}

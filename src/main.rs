mod components;
mod systems;
mod resources;
mod entities;
mod states;
mod utils;
mod ui;
mod debug;
mod units;
mod sprites;

use bevy::prelude::*;

// State plugins
use states::{
    loading::LoadingPlugin,
    main_menu::MainMenuPlugin,
    gameplay::GameplayPlugin,
    game_over::GameOverPlugin,
};

// System plugins
use crate::systems::{
    MovementPlugin,
    BaseMovePlugin,
    ModuleEffectsPlugin,
    ProductionPlugin,
    BaseInitializationPlugin,
    CameraPlugin,
};

// Component plugins
use crate::components::{
    AIPlugin,
    base_modules::BaseModulePlugin,
    strategic::StrategicLocationPlugin,
};

// UI plugins
use crate::ui::{
    BaseActionUIPlugin,
    BuildingProductionUIPlugin,
    BuildingSelectionUIPlugin,
};

// Resource plugins
use crate::resources::{
    map::plugin::MapPlugin,
    ResourceNodePlugin,
};

// Other plugins
use crate::debug::DebugPlugin;
use crate::units::EngineerPlugin;
use crate::sprites::SpriteLoaderPlugin;

// Re-export commonly used types for registration
use crate::components::{
    base_modules::{BaseModule, ModuleType, ResourceType, DamageType, UtilityEffect},
    player::MechanicalBase,
};
// Temporarily commented out for debugging
// use crate::utils::FontPlugin;
// use crate::components::UnitLabelPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    Gameplay,
    GameOver,
}

fn main() {
    App::new()
        // Add default Bevy plugins
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Strategy Forge".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            },
        ))
        // Add game state management
        .init_state::<GameState>()
        
        // Add our custom plugins individually
        // Game state management
        .add_plugins(LoadingPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(GameOverPlugin)
        
        // Core systems
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AIPlugin)
        .add_plugins(ResourceNodePlugin)
        .add_plugins(ProductionPlugin)
        .add_plugins(StrategicLocationPlugin)
        
        // Base systems
        .add_plugins(BaseModulePlugin)
        .add_plugins(BaseInitializationPlugin)
        .add_plugins(BaseMovePlugin)
        .add_plugins(ModuleEffectsPlugin)
        
        // Unit systems
        .add_plugins(EngineerPlugin)
        
        // UI systems
        .add_plugins(BaseActionUIPlugin)
        .add_plugins(BuildingProductionUIPlugin)
        .add_plugins(BuildingSelectionUIPlugin)
        
        // Debug and utility
        .add_plugins(DebugPlugin)
        .add_plugins(SpriteLoaderPlugin)
        
        // Register components for save/load (must be after all types are defined)
        .register_type::<MechanicalBase>()
        .register_type::<BaseModule>()
        .register_type::<ModuleType>()
        .register_type::<ResourceType>()
        .register_type::<DamageType>()
        .register_type::<UtilityEffect>()
        // Temporarily disable new plugins for debugging
        // .add_plugins(FontPlugin)
        // .add_plugins(UnitLabelPlugin)
        
        // Run the game
        .run();
}

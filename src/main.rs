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
    CameraManagerPlugin,
};

// Component plugins
use crate::components::{
    AIPlugin,
    base_modules::BaseModulePlugin,
    strategic::StrategicLocationPlugin,
    IsometricSpritePlugin,
};

// UI plugins
use crate::ui::{
    BaseActionUIPlugin,
    BuildingProductionUIPlugin,
    BuildingSelectionUIPlugin,
    menu::MenuPlugin,
};

// Resource plugins
use crate::resources::{
    map::plugin::MapPlugin,
    ResourceNodePlugin,
};

// Other plugins
use crate::debug::DebugPlugin;
use crate::entities::MobileBasePlugin;
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
        .init_state::<states::game_state::GameState>()
        
        // Add our custom plugins
        // Game state management
        .add_plugins(LoadingPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(GameOverPlugin)
        
        // Core systems - Add CameraManagerPlugin before CameraPlugin
        .add_plugins(CameraManagerPlugin) // Add this first to manage cameras
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
        // Temporarily disabled to prevent spawning steampunk bases on blank map
        // .add_plugins(MobileBasePlugin)
        
        // UI systems
        .add_plugins(BaseActionUIPlugin)
        .add_plugins(BuildingProductionUIPlugin)
        .add_plugins(BuildingSelectionUIPlugin)
        .add_plugins(MenuPlugin)
        
        // Debug and utility
        .add_plugins(DebugPlugin)
        .add_plugins(SpriteLoaderPlugin)
        .add_plugins(IsometricSpritePlugin)
        
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

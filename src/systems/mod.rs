//! Systems for the game

// Re-export all system modules
pub mod ai;
pub mod base_initialization;
pub mod base_movement;
pub mod camera;
pub mod camera_manager;
pub mod combat;
pub mod economy;
pub mod input;
pub mod module_effects;
pub mod movement;
pub mod production;
pub mod ui;

// Re-export commonly used items
pub use base_initialization::BaseInitializationPlugin;
pub use base_movement::BaseMovePlugin;
pub use camera::CameraPlugin;
pub use camera_manager::CameraManagerPlugin;
pub use module_effects::ModuleEffectsPlugin;
pub use movement::MovementPlugin;
pub use production::ProductionPlugin;

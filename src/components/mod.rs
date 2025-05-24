pub mod unit;
pub mod unit_label;
pub mod unit_types;
pub mod building;
pub mod resource;
pub mod terrain;
pub mod player;
pub mod ai;
pub mod strategic;
pub mod base_modules;

// Export plugins
// Temporarily commented out for debugging
// pub use unit_label::UnitLabelPlugin;
pub use ai::AIPlugin;

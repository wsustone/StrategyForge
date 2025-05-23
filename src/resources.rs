pub mod map;
pub mod map_data;
pub mod resource_nodes;

// These re-exports will be used when we implement the map systems
// For now, they're commented out to avoid compiler warnings
// pub use map_data::GameMap;
// pub use map_data::generate_map;
// pub use map_data::update_map_visibility;

// Re-export plugins
pub use resource_nodes::ResourceNodePlugin;

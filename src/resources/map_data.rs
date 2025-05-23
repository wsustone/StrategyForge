use bevy::prelude::*;

// Basic map data structures for Strategy Forge
#[derive(Resource)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Vec<TerrainType>>,
    pub initialized: bool,
    pub tile_size: f32,
    // Map of grid positions to tile entities
    pub tile_entities: Vec<Vec<Option<Entity>>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TerrainType {
    Plains,
    Forest,
    Hills,
    Mountains,
    Water,
}

impl Default for GameMap {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            terrain: vec![vec![TerrainType::Plains; 64]; 64],
            initialized: false,
            tile_size: 32.0,
            tile_entities: vec![vec![None; 64]; 64],
        }
    }
}

impl GameMap {
    // Check if a grid position is within the map bounds
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
    
    // Convert grid coordinates to world coordinates
    pub fn grid_to_world(&self, x: i32, y: i32) -> Vec2 {
        Vec2::new(
            x as f32 * self.tile_size,
            y as f32 * self.tile_size
        )
    }
    
    // Convert world coordinates to grid coordinates
    pub fn world_to_grid(&self, world_pos: Vec2) -> (i32, i32) {
        (
            (world_pos.x / self.tile_size).floor() as i32,
            (world_pos.y / self.tile_size).floor() as i32
        )
    }
    
    // Get the entity at a grid position
    pub fn get_tile_entity(&self, x: i32, y: i32) -> Option<Entity> {
        if self.is_in_bounds(x, y) {
            self.tile_entities[y as usize][x as usize]
        } else {
            None
        }
    }
    
    // Set the entity at a grid position
    pub fn set_tile_entity(&mut self, x: i32, y: i32, entity: Entity) {
        if self.is_in_bounds(x, y) {
            self.tile_entities[y as usize][x as usize] = Some(entity);
        }
    }
}

// Generate a simple map (placeholder implementation)
pub fn generate_map() -> GameMap {
    let mut map = GameMap::default();
    map.initialized = true;
    map
}

// Update map visibility (placeholder implementation)
pub fn update_map_visibility(_map: &mut GameMap) {
    // Placeholder for map visibility logic
}

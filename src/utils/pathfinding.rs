use bevy::prelude::*;
use pathfinding::prelude::astar;
use crate::resources::map_data::GameMap;
use crate::components::terrain::Terrain;

/// A position on the grid map with its cost
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    /// Calculate the manhattan distance to another position
    pub fn distance(&self, other: &GridPosition) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
    
    /// Get neighboring positions (4-way, not diagonal)
    pub fn neighbors(&self, game_map: &GameMap) -> Vec<(GridPosition, u32)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for (dx, dy) in directions.iter() {
            let nx = self.x + dx;
            let ny = self.y + dy;
            
            if game_map.is_in_bounds(nx, ny) {
                neighbors.push((GridPosition::new(nx, ny), 1));
            }
        }
        
        neighbors
    }
    
    /// Convert to world coordinates
    pub fn to_world(&self, game_map: &GameMap) -> Vec2 {
        game_map.grid_to_world(self.x, self.y)
    }
}

/// Find a path from start to goal
pub fn find_path(
    game_map: &GameMap,
    start: Vec2,
    goal: Vec2,
    terrain_query: &Query<(&Transform, &Terrain)>,
) -> Option<Vec<Vec2>> {
    let start_grid = game_map.world_to_grid(start);
    let goal_grid = game_map.world_to_grid(goal);
    
    let start_pos = GridPosition::new(start_grid.0, start_grid.1);
    let goal_pos = GridPosition::new(goal_grid.0, goal_grid.1);
    
    // A* search
    let result = astar(
        &start_pos,
        |p| {
            p.neighbors(game_map)
                .into_iter()
                .map(|(pos, cost)| {
                    // Check terrain for movement cost
                    let base_cost = cost;
                    let movement_cost = get_terrain_movement_cost(game_map, &pos, terrain_query);
                    let total_cost = (base_cost as f32 * movement_cost) as u32;
                    (pos, total_cost.max(1)) // Ensure cost is at least 1
                })
                .collect::<Vec<_>>()
        },
        |p| p.distance(&goal_pos) / 3, // Heuristic - divide by 3 to not overestimate
        |p| *p == goal_pos,
    );
    
    // Convert the path to world coordinates
    result.map(|(positions, _)| {
        positions.into_iter().map(|pos| pos.to_world(game_map)).collect()
    })
}

/// Get terrain movement cost at position
fn get_terrain_movement_cost(
    game_map: &GameMap,
    pos: &GridPosition,
    terrain_query: &Query<(&Transform, &Terrain)>,
) -> f32 {
    if let Some(_tile_entity) = game_map.get_tile_entity(pos.x, pos.y) {
        // Find terrain component
        for (_, terrain) in terrain_query.iter() {
            return 1.0 / terrain.movement_modifier.max(0.1);
        }
    }
    
    // Default cost if terrain not found
    1.0
}

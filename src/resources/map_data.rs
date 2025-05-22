use bevy::prelude::*;
// In Bevy 0.14.0, primitives are in different modules
use bevy::math::primitives::Rectangle;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::components::terrain::{TerrainType, Terrain, MapTile, StrategicPoint};
// use crate::components::unit::Team; // Unused for now - will be used for team-based visibility

// Resource that holds the entire game map
#[derive(Resource)]
pub struct GameMap {
    pub width: i32,
    pub height: i32,
    pub tile_size: f32,
    pub seed: u64,
    pub tiles: Vec<Entity>,
    pub strategic_locations: Vec<Entity>,
}

impl Default for GameMap {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            tile_size: 32.0,
            seed: 12345, // Default seed
            tiles: Vec::new(),
            strategic_locations: Vec::new(),
        }
    }
}

impl GameMap {
    pub fn new(width: i32, height: i32, tile_size: f32, seed: Option<u64>) -> Self {
        Self {
            width,
            height,
            tile_size,
            seed: seed.unwrap_or_else(|| rand::random()),
            tiles: Vec::with_capacity((width * height) as usize),
            strategic_locations: Vec::new(),
        }
    }

    // Convert grid coordinates to world coordinates
    pub fn grid_to_world(&self, grid_x: i32, grid_y: i32) -> Vec2 {
        Vec2::new(
            grid_x as f32 * self.tile_size - (self.width as f32 * self.tile_size / 2.0),
            grid_y as f32 * self.tile_size - (self.height as f32 * self.tile_size / 2.0),
        )
    }

    // Convert world coordinates to grid coordinates
    pub fn world_to_grid(&self, world_pos: Vec2) -> (i32, i32) {
        let offset_x = world_pos.x + (self.width as f32 * self.tile_size / 2.0);
        let offset_y = world_pos.y + (self.height as f32 * self.tile_size / 2.0);
        
        let grid_x = (offset_x / self.tile_size).floor() as i32;
        let grid_y = (offset_y / self.tile_size).floor() as i32;
        
        (grid_x, grid_y)
    }

    // Check if grid coordinates are within map bounds
    pub fn is_in_bounds(&self, grid_x: i32, grid_y: i32) -> bool {
        grid_x >= 0 && grid_x < self.width && grid_y >= 0 && grid_y < self.height
    }

    // Get index in the tiles vector from grid coordinates
    pub fn get_index(&self, grid_x: i32, grid_y: i32) -> Option<usize> {
        if self.is_in_bounds(grid_x, grid_y) {
            Some((grid_y * self.width + grid_x) as usize)
        } else {
            None
        }
    }

    // Get the entity at the specified grid coordinates
    pub fn get_tile_entity(&self, grid_x: i32, grid_y: i32) -> Option<Entity> {
        self.get_index(grid_x, grid_y).map(|idx| self.tiles[idx])
    }
}

// System to generate a procedural map
pub fn generate_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    seed: Option<u64>,
) -> GameMap {
    // Create a new map with the specified dimensions
    let map_width = 100;
    let map_height = 100;
    let tile_size = 32.0;
    
    let mut game_map = GameMap::new(map_width, map_height, tile_size, seed);
    
    // Initialize RNG with seed
    let mut rng = StdRng::seed_from_u64(game_map.seed);
    
    // Generate terrain using simplex noise or another algorithm
    // For now, we'll use a simple random approach for illustration
    let mut tiles = Vec::with_capacity((map_width * map_height) as usize);
    
    // Create a rectangle mesh for tiles using Bevy 0.14.0's approach
    let _tile_mesh = meshes.add(Mesh::from(Rectangle::new(tile_size, tile_size)));
    
    // Create materials for different terrain types
    let _terrain_materials = create_terrain_materials(&mut materials);
    
    // Place strategic locations (3-5 of them)
    let num_strategic_locations = rng.gen_range(3..=5);
    let mut strategic_locations = Vec::new();
    
    for _ in 0..num_strategic_locations {
        let x = rng.gen_range(5..map_width - 5);
        let y = rng.gen_range(5..map_height - 5);
        strategic_locations.push((x, y));
    }
    
    // Generate the map tiles
    for y in 0..map_height {
        for x in 0..map_width {
            // Determine terrain type
            let terrain_type = determine_terrain_type(x, y, &strategic_locations, &mut rng);
            let terrain = Terrain::new(terrain_type);
            
            // Create the tile entity
            let world_pos = game_map.grid_to_world(x, y);
            
            let tile_entity = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        // Use a direct color instead of trying to convert from ColorMaterial handle
                        color: get_terrain_color(terrain_type),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                    ..default()
                },
                terrain,
                MapTile {
                    grid_x: x,
                    grid_y: y,
                    is_explored: false,
                    is_visible: false,
                },
            )).id();
            
            // If this is a strategic location, add the StrategicPoint component
            if strategic_locations.contains(&(x, y)) {
                let world_pos = game_map.grid_to_world(x, y);
                commands.entity(tile_entity).insert(StrategicPoint {
                    capture_progress: 0.0,
                    controlling_team: None,
                    capture_value: 100,
                    position: world_pos,
                });
                
                game_map.strategic_locations.push(tile_entity);
            }
            
            tiles.push(tile_entity);
        }
    }
    
    game_map.tiles = tiles;
    game_map
}

// Function to get a color for a terrain type directly
fn get_terrain_color(terrain_type: TerrainType) -> Color {
    match terrain_type {
        TerrainType::Plains => Color::srgb(0.7, 0.8, 0.3),     // Light green
        TerrainType::Forest => Color::srgb(0.0, 0.5, 0.0),     // Dark green
        TerrainType::Mountain => Color::srgb(0.5, 0.5, 0.5),   // Gray
        TerrainType::Water => Color::srgb(0.0, 0.3, 0.8),      // Blue
        TerrainType::MetalDeposit => Color::srgb(0.6, 0.6, 0.7), // Silver
        TerrainType::RoughTerrain => Color::srgb(0.4, 0.3, 0.2), // Brown
        TerrainType::StrategicLocation => Color::srgb(0.9, 0.5, 0.1), // Orange
    }
}

fn create_terrain_materials(
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> std::collections::HashMap<TerrainType, Handle<ColorMaterial>> {
    let mut terrain_materials = std::collections::HashMap::new();
    
    // Create materials for each terrain type with appropriate colors
    terrain_materials.insert(
        TerrainType::Plains,
        materials.add(ColorMaterial::from(Color::srgb(0.7, 0.8, 0.3))), // Light green
    );
    
    terrain_materials.insert(
        TerrainType::Forest,
        materials.add(ColorMaterial::from(Color::srgb(0.0, 0.5, 0.0))), // Dark green
    );
    
    terrain_materials.insert(
        TerrainType::Mountain,
        materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.5))), // Gray
    );
    
    terrain_materials.insert(
        TerrainType::Water,
        materials.add(ColorMaterial::from(Color::srgb(0.0, 0.3, 0.8))), // Blue
    );
    
    terrain_materials.insert(
        TerrainType::MetalDeposit,
        materials.add(ColorMaterial::from(Color::srgb(0.6, 0.6, 0.6))), // Silver
    );
    
    terrain_materials.insert(
        TerrainType::RoughTerrain,
        materials.add(ColorMaterial::from(Color::srgb(0.6, 0.5, 0.3))), // Brown
    );
    
    terrain_materials.insert(
        TerrainType::StrategicLocation,
        materials.add(ColorMaterial::from(Color::srgb(1.0, 0.8, 0.0))), // Gold
    );
    
    terrain_materials
}

fn determine_terrain_type(
    x: i32,
    y: i32,
    strategic_locations: &Vec<(i32, i32)>,
    rng: &mut StdRng,
) -> TerrainType {
    // If this is a strategic location
    if strategic_locations.contains(&(x, y)) {
        return TerrainType::StrategicLocation;
    }
    
    // Simple random terrain generation
    // In a real implementation, you'd want to use noise functions for more natural terrain
    let terrain_roll = rng.gen_range(0..100);
    
    match terrain_roll {
        0..=50 => TerrainType::Plains,    // 51% chance - most common
        51..=70 => TerrainType::Forest,   // 20% chance
        71..=80 => TerrainType::RoughTerrain, // 10% chance
        81..=88 => TerrainType::Mountain, // 8% chance
        89..=94 => TerrainType::Water,    // 6% chance
        _ => TerrainType::MetalDeposit,   // 5% chance - most rare
    }
}

// System to update map visibility based on unit positions
pub fn update_map_visibility(
    _commands: Commands, // Unused, prefixed with underscore
    game_map: Res<GameMap>,
    units: Query<(&Transform, &crate::components::unit::Unit)>,
    mut tiles: Query<(&MapTile, &mut Sprite)>,
) {
    // Reset visibility
    for (_tile, mut sprite) in &mut tiles {
        sprite.color = sprite.color.with_alpha(0.5); // Semi-transparent for non-visible tiles
    }
    
    // Update visibility based on unit positions
    for (transform, _unit) in &units {
        let unit_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let (grid_x, grid_y) = game_map.world_to_grid(unit_pos);
        
        // Simple visibility radius (can be adjusted based on unit type, terrain, etc.)
        let visibility_radius = 5;
        
        // Mark tiles as visible
        for y in (grid_y - visibility_radius)..(grid_y + visibility_radius + 1) {
            for x in (grid_x - visibility_radius)..(grid_x + visibility_radius + 1) {
                if let Some(tile_entity) = game_map.get_tile_entity(x, y) {
                    if let Ok((_tile, mut sprite)) = tiles.get_mut(tile_entity) {
                        // Calculate distance for circular visibility
                        let distance = ((x - grid_x).pow(2) + (y - grid_y).pow(2)) as f32;
                        if distance <= (visibility_radius * visibility_radius) as f32 {
                            sprite.color = sprite.color.with_alpha(1.0); // Fully visible
                        }
                    }
                }
            }
        }
    }
}

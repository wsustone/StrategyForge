use bevy::prelude::*;
use crate::GameState;
use crate::resources::map_data::{GameMap, generate_map, update_map_visibility};
use crate::components::unit::Team;
use crate::components::terrain::StrategicPoint;
// use crate::components::terrain::TerrainType; // Unused for now
use crate::components::player::MechanicalBase;
use crate::components::player::PlayerResources;
use crate::components::ai::{AIControlled, AIBase, AIDifficulty};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Gameplay), setup_map)
            .add_systems(
                Update,
                (
                    update_map_visibility,
                    update_strategic_points,
                    check_strategic_point_ownership,
                ).run_if(in_state(GameState::Gameplay))
            )
            .add_systems(OnExit(GameState::Gameplay), cleanup_map);
    }
}

// Setup the game map when entering gameplay state
pub fn setup_map(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    // Generate a new random map
    let game_map = generate_map(commands.reborrow(), meshes, materials, None);
    
    // Spawn the initial base locations for players and AI
    spawn_starting_bases(&mut commands, &game_map);
    
    // Insert the map as a resource after using it
    commands.insert_resource(game_map);
    
    // Create fog of war overlay
    // This would typically be a semi-transparent overlay covering unexplored areas
}

// Spawn the starting mechanical bases for player and AI
fn spawn_starting_bases(commands: &mut Commands, game_map: &GameMap) {
    // Define base starting positions - opposite corners of the map
    let player_position = (5, 5); // Bottom left (player)
    let ai_position = (game_map.width - 5, game_map.height - 5); // Top right (AI)
    
    // Spawn player's mechanical base
    let player_world_pos = game_map.grid_to_world(player_position.0, player_position.1);
    let player_color = Color::srgb(0.2, 0.6, 0.8); // Blue color for player
    
    info!("Spawning player base at position: {:?}", player_world_pos);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: player_color,
                custom_size: Some(Vec2::new(game_map.tile_size * 2.0, game_map.tile_size * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(player_world_pos.x, player_world_pos.y, 10.0), // Above terrain
            ..default()
        },
        MechanicalBase {
            health: 1000.0,
            max_health: 1000.0,
            movement_speed: 10.0, // Slow movement
            team: Team::Player,
            resources: vec![
                (crate::components::building::ResourceType::Wood, 200),
                (crate::components::building::ResourceType::Stone, 100),
                (crate::components::building::ResourceType::Iron, 50),
            ],
        },
        Name::new("Player Base"),
    ));
    
    // Initialize player resources
    commands.insert_resource(PlayerResources::default());
    
    // Spawn AI's mechanical base
    let ai_world_pos = game_map.grid_to_world(ai_position.0, ai_position.1);
    let ai_color = Color::srgb(0.8, 0.2, 0.2); // Red color for AI
    
    info!("Spawning AI base at position: {:?}", ai_world_pos);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ai_color,
                custom_size: Some(Vec2::new(game_map.tile_size * 2.0, game_map.tile_size * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(ai_world_pos.x, ai_world_pos.y, 10.0), // Above terrain
            ..default()
        },
        MechanicalBase {
            health: 1000.0,
            max_health: 1000.0,
            movement_speed: 10.0, // Slow movement
            team: Team::Enemy,
            resources: vec![
                (crate::components::building::ResourceType::Wood, 200),
                (crate::components::building::ResourceType::Stone, 100),
                (crate::components::building::ResourceType::Iron, 50),
            ],
        },
        AIControlled {
            difficulty: AIDifficulty::Medium,
        },
        AIBase,
        Name::new("AI Base"),
    ));
}

// System to update control of strategic points
fn update_strategic_points(
    _commands: Commands, // Not used directly, renamed with underscore
    game_map: Res<GameMap>,
    time: Res<Time>,
    mut strategic_points: Query<(Entity, &mut StrategicPoint)>,
    units: Query<(&Transform, &crate::components::unit::Unit)>,
) {
    let capture_radius = 3; // Units within this radius can capture a point
    
    for (_entity, mut strategic_point) in &mut strategic_points {
        // Skip if already fully captured
        if strategic_point.capture_progress >= 1.0 && strategic_point.controlling_team.is_some() {
            continue;
        }
        
        // Count units by team within capture radius
        let mut player_units = 0;
        let mut enemy_units = 0;
        
        // We'll use a simpler approach here - just use the strategic point's world position
        // directly instead of trying to find the tile entity
        let world_pos = strategic_point.position;
        let (grid_x, grid_y) = game_map.world_to_grid(world_pos);
                
        // Check units in capture radius
        for (transform, unit) in &units {
            let unit_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let (unit_grid_x, unit_grid_y) = game_map.world_to_grid(unit_pos);
            
            let distance = ((unit_grid_x - grid_x).pow(2) + (unit_grid_y - grid_y).pow(2)) as f32;
            if distance <= (capture_radius * capture_radius) as f32 {
                match unit.team {
                    Team::Player => player_units += 1,
                    Team::Enemy => enemy_units += 1,
                    Team::Neutral => {}, // Neutral units don't affect capture
                }
            }
        }
        
        // Update capture progress based on unit presence
        let capture_rate = 0.1 * time.delta_seconds(); // Base capture rate
        
        if player_units > enemy_units {
            // Player is capturing
            strategic_point.capture_progress += capture_rate * player_units as f32;
            if strategic_point.capture_progress >= 1.0 {
                strategic_point.capture_progress = 1.0;
                strategic_point.controlling_team = Some(Team::Player);
            }
        } else if enemy_units > player_units {
            // Enemy is capturing
            strategic_point.capture_progress += capture_rate * enemy_units as f32;
            if strategic_point.capture_progress >= 1.0 {
                strategic_point.capture_progress = 1.0;
                strategic_point.controlling_team = Some(Team::Enemy);
            }
        } else if player_units == 0 && enemy_units == 0 {
            // No units present, slowly decay capture progress
            strategic_point.capture_progress = (strategic_point.capture_progress - 0.05 * time.delta_seconds()).max(0.0);
            if strategic_point.capture_progress <= 0.0 {
                strategic_point.controlling_team = None;
            }
        }
    }
}

// System to check overall control of strategic points for victory conditions
fn check_strategic_point_ownership(
    strategic_points: Query<&StrategicPoint>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut player_points = 0;
    let mut enemy_points = 0;
    let mut total_points = 0;
    
    for point in &strategic_points {
        total_points += 1;
        if let Some(team) = &point.controlling_team {
            match team {
                Team::Player => player_points += 1,
                Team::Enemy => enemy_points += 1,
                _ => {},
            }
        }
    }
    
    // Victory condition: control majority of points
    if total_points > 0 {
        let threshold = (total_points as f32 * 0.7).ceil() as i32; // Need 70% control
        
        if player_points >= threshold {
            // Player wins - would transition to victory state
            // For now, just go back to main menu
            game_state.set(GameState::MainMenu);
        } else if enemy_points >= threshold {
            // Enemy wins
            game_state.set(GameState::GameOver);
        }
    }
}

// Clean up map when exiting gameplay state
fn cleanup_map(
    mut commands: Commands,
    game_map: Res<GameMap>,
) {
    // Despawn all map tiles
    for entity in &game_map.tiles {
        commands.entity(*entity).despawn_recursive();
    }
    
    // Remove the game map resource
    commands.remove_resource::<GameMap>();
}

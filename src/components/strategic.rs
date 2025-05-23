use bevy::prelude::*;
use crate::components::unit::Team;
use crate::components::player::MechanicalBase;
use crate::resources::map_data::GameMap;
use crate::resources::map::plugin::MapInitialized;
use crate::GameState;
use rand::Rng;

/// Component representing a strategic location target
#[derive(Component)]
pub struct StrategicLocation {
    pub name: String,
    pub control_points: f32,
    pub total_required: f32,
    pub controlling_team: Option<Team>,
    pub position: Vec2,
}

impl Default for StrategicLocation {
    fn default() -> Self {
        Self {
            name: "Strategic Location".to_string(),
            control_points: 0.0,
            total_required: 100.0, // Points needed to capture
            controlling_team: None,
            position: Vec2::ZERO,
        }
    }
}

/// Marker component for the visual indicator of a strategic location
#[derive(Component)]
pub struct StrategicLocationMarker;

/// Plugin to manage strategic locations
pub struct StrategicLocationPlugin;

impl Plugin for StrategicLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_strategic_locations.after(crate::resources::map::plugin::setup_map))
           .add_systems(
                Update, 
                (
                    update_strategic_location_control,
                    update_strategic_location_visuals,
                ).run_if(in_state(GameState::Gameplay))
            );
        
        info!("Strategic Location Plugin initialized");
    }
}

/// System to spawn strategic locations on the map
fn spawn_strategic_locations(
    mut commands: Commands,
    game_map: Res<GameMap>,
    map_initialized: Res<MapInitialized>,
) {
    // Only spawn strategic locations if the map is already initialized
    // This prevents duplicate spawning when returning to gameplay
    if map_initialized.0 {
        info!("Spawning strategic locations");
    } else {
        info!("Map not yet initialized, skipping strategic location spawning");
        return;
    }
    // Define the number of strategic locations based on the number of players
    // For now we hardcode 2 players, so 1 strategic location (2/2 = 1)
    let num_locations = 1;
    
    let mut rng = rand::thread_rng();
    
    // Create strategic locations in the center area of the map
    for i in 0..num_locations {
        // Choose a position in the central area of the map
        let center_x = game_map.width as f32 / 2.0;
        let center_y = game_map.height as f32 / 2.0;
        
        // Add some randomness to the position, but keep it near the center
        let offset_x = rng.gen_range(-5.0..5.0);
        let offset_y = rng.gen_range(-5.0..5.0);
        
        let grid_x = (center_x + offset_x).round() as i32;
        let grid_y = (center_y + offset_y).round() as i32;
        
        // Convert grid position to world position
        let world_pos = game_map.grid_to_world(grid_x, grid_y);
        
        // Create the strategic location
        let _location = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.9, 0.7, 0.1, 1.0), // Gold color
                    custom_size: Some(Vec2::new(game_map.tile_size * 3.0, game_map.tile_size * 3.0)),
                    ..default()
                },
                transform: Transform::from_xyz(world_pos.x, world_pos.y, 5.0), // Above terrain, below units
                ..default()
            },
            StrategicLocation {
                name: format!("Strategic Point {}", i + 1),
                position: Vec2::new(world_pos.x, world_pos.y),
                ..default()
            },
            Name::new(format!("Strategic Location {}", i + 1)),
        )).id();
        
        // Add a visual marker to make it more visible
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 0.9, 0.3, 1.0), // Brighter gold for the marker
                    custom_size: Some(Vec2::new(game_map.tile_size * 1.5, game_map.tile_size * 1.5)),
                    ..default()
                },
                transform: Transform::from_xyz(world_pos.x, world_pos.y, 6.0), // Above the base sprite
                ..default()
            },
            StrategicLocationMarker,
            Name::new(format!("Strategic Location Marker {}", i + 1)),
        ));
        
        info!("Spawned Strategic Location {} at position: {:?}", i + 1, world_pos);
    }
}

/// System to update the control status of strategic locations
fn update_strategic_location_control(
    time: Res<Time>,
    mut locations: Query<&mut StrategicLocation>,
    bases: Query<(&Transform, &MechanicalBase)>,
) {
    // Define the capture radius
    let capture_radius = 100.0;
    
    for mut location in locations.iter_mut() {
        // Check which teams have bases within the capture radius
        let mut controlling_teams: Vec<Team> = Vec::new();
        
        for (transform, base) in bases.iter() {
            let base_pos = transform.translation.truncate();
            let distance = location.position.distance(base_pos);
            
            if distance < capture_radius {
                controlling_teams.push(base.team);
            }
        }
        
        // Update control points based on present teams
        if controlling_teams.len() == 1 {
            // Only one team present, they gain control
            let team = controlling_teams[0];
            
            // If this is a different team than the current controlling team, reset points
            if location.controlling_team.is_some() && location.controlling_team != Some(team) {
                location.control_points = 0.0;
                location.controlling_team = None;
            }
            
            // Increase control points for this team
            location.control_points += time.delta_seconds() * 10.0; // 10 points per second
            
            // Cap at max points
            if location.control_points >= location.total_required {
                location.control_points = location.total_required;
                location.controlling_team = Some(team);
                
                info!("Team {:?} has captured location: {}", team, location.name);
            }
        } else if controlling_teams.len() > 1 {
            // Multiple teams present, control is contested
            // Slowly decrease control points when contested
            location.control_points = (location.control_points - time.delta_seconds() * 5.0).max(0.0);
            
            if location.control_points <= 0.0 {
                location.controlling_team = None;
            }
        } else {
            // No teams present, slowly decay control
            location.control_points = (location.control_points - time.delta_seconds() * 2.0).max(0.0);
            
            if location.control_points <= 0.0 {
                location.controlling_team = None;
            }
        }
    }
}

/// System to update the visual appearance of strategic locations
fn update_strategic_location_visuals(
    mut param_set: ParamSet<(
        Query<(&StrategicLocation, &mut Sprite)>,
        Query<(&mut Sprite, &GlobalTransform), With<StrategicLocationMarker>>
    )>,
) {
    // Use the first query from the ParamSet
    let mut locations = param_set.p0();
    for (location, mut sprite) in locations.iter_mut() {
        // Update the strategic location sprite based on control status
        if let Some(team) = location.controlling_team {
            match team {
                Team::Player => sprite.color = Color::srgba(0.2, 0.6, 0.8, 1.0), // Blue for player control
                Team::Enemy => sprite.color = Color::srgba(0.8, 0.2, 0.2, 1.0),  // Red for enemy control
                _ => sprite.color = Color::srgba(0.9, 0.7, 0.1, 1.0),           // Default gold
            }
        } else {
            // No controlling team, use default color with alpha based on control points
            // We're not using control_percent for now, just keeping the default color
            sprite.color = Color::srgba(0.9, 0.7, 0.1, 1.0);
        }
    }
}

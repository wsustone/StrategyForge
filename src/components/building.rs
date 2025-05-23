use bevy::prelude::*;

#[derive(Component)]
pub struct Building {
    pub health: f32,
    pub max_health: f32,
    pub construction_progress: f32,
    pub is_completed: bool,
}

#[derive(Component)]
pub struct BuildingSpawner {
    pub unit_type: String,
    pub spawn_time: f32,
    pub spawn_timer: Timer,
}

#[derive(Component)]
pub struct ResourceGenerator {
    pub resource_type: ResourceType,
    pub generation_rate: f32,
    pub generation_timer: Timer,
}

#[derive(Component)]
pub struct Constructable {
    pub construction_time: f32,
    pub resource_cost: Vec<(ResourceType, i32)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ResourceType {
    Wood,
    Stone,
    Iron,
}

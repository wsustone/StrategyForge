use bevy::prelude::*;
use crate::components::building::ResourceType;

#[derive(Component)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub amount_remaining: i32,
    pub max_gatherers: i32,
    pub current_gatherers: i32,
}

#[derive(Component)]
pub struct Gatherer {
    pub gather_rate: f32,
    pub gather_timer: Timer,
    pub carry_capacity: i32,
    pub current_load: i32,
    pub target_resource: Option<Entity>,
}

use bevy::prelude::*;
use bevy::ecs::entity::Entity;

// Unit components for our RTS game

#[derive(Component)]
pub struct Unit {
    pub health: f32,
    pub max_health: f32,
    pub attack_power: f32,
    pub attack_range: f32,
    pub movement_speed: f32,
    pub team: Team,
    pub state: UnitState,
    pub attack_cooldown: Timer,
    pub attack_target: Option<Entity>,
    pub movement_target: Option<Vec2>,
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub enum UnitState {
    Idle,
    Moving,
    Attacking,
    Gathering,
    Building,
}

use bevy::reflect::Reflect;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Reflect)]
#[reflect(Component)]
pub enum Team {
    Player,
    Enemy,
    Neutral,
}

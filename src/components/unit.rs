use bevy::prelude::*;

// Unit components for our RTS game
#[derive(Component)]
pub struct Unit {
    pub health: f32,
    pub max_health: f32,
    pub attack_power: f32,
    pub attack_range: f32,
    pub movement_speed: f32,
    pub team: Team,
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

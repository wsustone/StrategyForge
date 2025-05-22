use bevy::prelude::*;
// Will be used when implementing angle-based calculations
// use std::f32::consts::PI;

/// Distance between two Vec2 points
pub fn distance(p1: Vec2, p2: Vec2) -> f32 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

/// Direction vector from p1 to p2, normalized
pub fn direction(from: Vec2, to: Vec2) -> Vec2 {
    let delta = to - from;
    if delta.length() > 0.0 {
        delta.normalize()
    } else {
        Vec2::ZERO
    }
}

/// Convert a direction vector to an angle in radians
pub fn vector_to_angle(direction: Vec2) -> f32 {
    direction.y.atan2(direction.x)
}

/// Convert an angle in radians to a direction vector
pub fn angle_to_vector(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

/// Limit a value between min and max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between a and b by t
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Check if a point is inside a rectangle
pub fn point_in_rect(point: Vec2, rect_pos: Vec2, rect_size: Vec2) -> bool {
    let half_width = rect_size.x / 2.0;
    let half_height = rect_size.y / 2.0;
    
    point.x >= rect_pos.x - half_width &&
    point.x <= rect_pos.x + half_width &&
    point.y >= rect_pos.y - half_height &&
    point.y <= rect_pos.y + half_height
}

/// Calculate a 2D perlin noise value (for map generation)
pub fn perlin_noise_2d(x: f32, y: f32, seed: u32) -> f32 {
    // Simple implementation - in a real game you'd use a proper noise library
    let x = x * 0.1 + seed as f32;
    let y = y * 0.1;
    
    let a = (x * 12.9898 + y * 78.233).sin() * 43758.5453;
    let b = (x * 39.346 + y * 11.135).sin() * 53758.5453;
    
    (a.sin() + b.sin()).abs() % 1.0
}

use bevy::prelude::*;
use crate::components::player::MechanicalBase;
use crate::components::base_modules::{
    BaseModule, ModuleType, DamageType, UtilityEffect, ResourceType
};
use crate::components::unit::Team;

/// System to manage module activation/deactivation based on power availability
pub fn manage_module_power(
    mut bases: Query<(&mut MechanicalBase, &Children)>,
    mut modules: Query<(&mut BaseModule, &mut Sprite)>,
) {
    for (base, children) in &mut bases {
        let mut available_power = base.power_output - base.power_consumed;
        
        // First pass: Deactivate all modules to start with a clean slate
        for &child in children.iter() {
            if let Ok((mut module, _)) = modules.get_mut(child) {
                module.active = false;
            }
        }
        
        // Second pass: Activate modules in priority order until we run out of power
        // Priority order: 1. Defense, 2. Movement, 3. Sensors, 4. Weapons, 5. Production, 6. Storage, 7. Utility
        let module_order = [
            ModuleType::Defense { armor_bonus: 0.0, shield_strength: 0.0, shield_recharge_rate: 0.0, damage_resistance: 0.0 },
            ModuleType::Movement { speed_modifier: 0.0, efficiency: 0.0, terrain_penalty_reduction: 0.0 },
            ModuleType::Sensor { detection_radius: 0.0, stealth_detection: 0.0, vision_range: 0.0, scan_cooldown: 0.0 },
            ModuleType::Weapon { damage: 0.0, attack_speed: 0.0, range: 0.0, damage_type: DamageType::Kinetic, splash_radius: 0.0, tracking_speed: 0.0 },
            ModuleType::Production { build_speed: 0.0, queue_slots: 0, cost_reduction: 0.0, experience_gain: 0.0 },
            ModuleType::Storage { capacity: 0, resource_type: ResourceType::Basic, passive_generation: 0.0 },
            ModuleType::Utility { effect_type: UtilityEffect::Repair, effect_strength: 0.0, area_of_effect: 0.0, cooldown: 0.0 },
        ];
        
        for module_type in module_order.iter() {
            for &child in children.iter() {
                if available_power <= 0.0 { break; }
                
                if let Ok((mut module, mut sprite)) = modules.get_mut(child) {
                    if !module.active && std::mem::discriminant(&module.module_type) == std::mem::discriminant(module_type) {
                        if available_power >= module.power_consumption {
                            module.active = true;
                            available_power -= module.power_consumption;
                            // Update visual state - use white for active modules
                            sprite.color = Color::WHITE;
                        } else {
                            // Not enough power, keep inactive - use gray (sRGB 0.5, 0.5, 0.5) for inactive modules
                            module.active = false;
                            sprite.color = Color::srgb(0.5, 0.5, 0.5);
                        }
                    }
                }
            }
        }
    }
}

/// System to apply module effects to the base each frame
pub fn apply_module_effects(
    time: Res<Time>,
    mut bases: Query<(&mut MechanicalBase, &Children)>,
    modules: Query<(&BaseModule, &Sprite)>,
) {
    let delta = time.delta_seconds();
    
    for (mut base, children) in &mut bases {
        // Reset base stats that are modified by modules
        let mut effective_stats = BaseStats::default();
        
        // Apply effects from all child modules
        for &child in children.iter() {
            if let Ok((module, _)) = modules.get(child) {
                if !module.active { continue; }
                
                match &module.module_type {
                    ModuleType::Movement {
                        speed_modifier,
                        efficiency: _,
                        terrain_penalty_reduction,
                    } => {
                        effective_stats.speed_multiplier *= speed_modifier;
                        effective_stats.terrain_penalty_reduction = effective_stats
                            .terrain_penalty_reduction
                            .max(*terrain_penalty_reduction);
                    }
                    ModuleType::Defense {
                        armor_bonus,
                        shield_strength,
                        shield_recharge_rate,
                        damage_resistance,
                    } => {
                        effective_stats.armor += *armor_bonus;
                        effective_stats.max_shield = effective_stats
                            .max_shield
                            .max(*shield_strength);
                        effective_stats.shield_recharge_rate += *shield_recharge_rate;
                        effective_stats.damage_resistance = (effective_stats.damage_resistance
                            + *damage_resistance)
                            .min(0.9); // Cap at 90% damage resistance
                    }
                    ModuleType::Energy {
                        power_output,
                        power_capacity,
                        efficiency: _,
                        power_transfer_rate: _,
                    } => {
                        // Handle power generation
                        effective_stats.power_generated += *power_output;
                        effective_stats.power_capacity += *power_capacity;
                    }
                    ModuleType::Weapon { .. } => {
                        // Weapon targeting is handled in a separate system
                        effective_stats.has_weapons = true;
                    }
                    ModuleType::Sensor { .. } => {
                        // Sensor effects are handled in the sensor system
                    }
                    ModuleType::Production { .. } => {
                        // Production effects are handled in the production system
                    }
                    ModuleType::Storage { .. } => {
                        // Storage effects are handled in the resource system
                    }
                    ModuleType::Utility { .. } => {
                        // Utility effects are handled in the utility system
                    }
                }
                
                // Apply module power consumption
                if module.active {
                    effective_stats.power_consumed += module.power_consumption;
                }
            }
        }
        
        // Apply the calculated stats to the base
        base.effective_movement_speed = base.base_movement_speed * effective_stats.speed_multiplier;
        
        // Update power stats - ensure we don't go below 0
        base.power_output = effective_stats.power_generated.max(0.0);
        base.power_consumed = effective_stats.power_consumed.max(0.0);
        base.max_power = 150.0 + effective_stats.power_capacity; // Base 150 + module bonuses
        
        // Ensure we don't have more power than capacity
        base.power_consumed = base.power_consumed.min(base.max_power);
    }
}

/// System to handle weapon module targeting and firing
pub fn handle_weapon_modules(
    _time: Res<Time>,
    _commands: Commands,
    _bases: Query<(&Transform, &Team, &Children), With<MechanicalBase>>,
    _modules: Query<(&mut BaseModule, &GlobalTransform, &mut Cooldown)>,
    _targets: Query<(&Transform, &Team)>
) {
    // Implementation will be added later
    // This function will handle weapon module targeting and firing logic
    // when the combat system is implemented
}

/// System to handle utility module effects
pub fn handle_utility_modules(
    _time: Res<Time>,
    _commands: Commands,
    _bases: Query<(&Transform, &Team, &Children), With<MechanicalBase>>,
    _modules: Query<(&BaseModule, &GlobalTransform, &mut Cooldown)>,
) {
}

// Helper components and structs

/// Tracks cooldown for module abilities
#[derive(Component)]
pub struct Cooldown {
    pub timer: Timer,
}

/// Projectile component for weapon modules
#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub damage_type: DamageType,
    pub splash_radius: f32,
    pub speed: f32,
    pub target: Vec3,
    pub source: Vec3,
}

/// Visual/sound effect for abilities
#[derive(Component)]
pub struct Effect {
    pub effect_type: UtilityEffect,
    pub position: Vec3,
    pub duration: f32,
    pub strength: f32,
}

/// Health component for damageable entities
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub shield: f32,
    pub max_shield: f32,
}

/// Tracks effective stats after applying all module effects
#[derive(Default)]
struct BaseStats {
    speed_multiplier: f32,
    armor: f32,
    max_shield: f32,
    shield_recharge_rate: f32,
    damage_resistance: f32,
    power_generated: f32,
    power_consumed: f32,
    power_capacity: f32,  // Added missing field
    has_weapons: bool,
    terrain_penalty_reduction: f32,
}

/// Plugin for module systems
pub struct ModuleEffectsPlugin;

impl Plugin for ModuleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // First manage power to determine which modules should be active
                manage_module_power,
                // Then apply the effects of active modules
                apply_module_effects,
                // Finally handle module-specific behaviors
                (handle_weapon_modules, handle_utility_modules),
            )
            .chain(),
        );
    }
}

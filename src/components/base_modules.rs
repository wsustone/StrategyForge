use bevy::prelude::*;
use bevy::reflect::Reflect;
use crate::components::unit::Team;
use crate::components::player::MechanicalBase;

/// Represents a module that can be attached to the mechanical base
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct BaseModule {
    pub module_type: ModuleType,
    pub health: f32,
    pub max_health: f32,
    pub power_consumption: f32,
    pub active: bool,
    pub team: Team,
}

/// Types of base modules with their specific properties
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum ModuleType {
    // Movement modules - enhance base mobility
    Movement {
        speed_modifier: f32,      // Multiplier for base movement speed (e.g., 1.5 for +50% speed)
        efficiency: f32,          // Power efficiency (0.0-1.0, affects power consumption)
        terrain_penalty_reduction: f32, // Reduces speed penalty on rough terrain (0.0-1.0)
    },
    
    // Storage modules - increase resource capacity
    Storage {
        capacity: i32,            // Additional resource storage capacity
        resource_type: ResourceType, // Type of resource stored
        passive_generation: f32,    // Passive generation per second (if any)
    },
    
    // Defense modules - improve base survivability
    Defense {
        armor_bonus: f32,         // Additional armor points (flat reduction to damage)
        shield_strength: f32,     // Shield hit points (0 if no shield)
        shield_recharge_rate: f32, // Shield points regenerated per second
        damage_resistance: f32,    // Percentage damage reduction (0.0-1.0)
    },
    
    // Production modules - enhance unit/building production
    Production {
        build_speed: f32,         // Build speed multiplier (e.g., 1.2 for +20% speed)
        queue_slots: u8,          // Additional production queue slots
        cost_reduction: f32,       // Percentage reduction in resource costs (0.0-1.0)
        experience_gain: f32,      // Bonus experience for produced units
    },
    
    // Sensor modules - improve detection and awareness
    Sensor {
        detection_radius: f32,     // Radius for detecting enemy units
        stealth_detection: f32,    // Bonus against stealthed units (0.0-1.0)
        vision_range: f32,         // Bonus to vision range
        scan_cooldown: f32,        // Cooldown between sensor pings
    },
    
    // Energy modules - power generation and management
    Energy {
        power_output: f32,        // Additional power generated
        power_capacity: f32,       // Additional power storage
        efficiency: f32,           // Efficiency of power generation (affects fuel consumption)
        power_transfer_rate: f32,  // How quickly power can be transferred to other bases
    },
    
    // Weapon modules - offensive capabilities
    Weapon {
        damage: f32,              // Base damage per shot
        attack_speed: f32,         // Attacks per second
        range: f32,                // Attack range in world units
        damage_type: DamageType,    // Type of damage (kinetic, energy, explosive, etc.)
        splash_radius: f32,        // Radius of splash damage (0 for single target)
        tracking_speed: f32,       // How quickly it can track moving targets
    },
    
    // Utility modules - various support functions
    Utility {
        effect_type: UtilityEffect, // Type of utility effect
        effect_strength: f32,      // Strength of the effect
        area_of_effect: f32,       // Radius of effect
        cooldown: f32,             // Cooldown between uses
    },
}

/// Component marking an attachment point on the base
#[derive(Component, Debug)]
pub struct AttachmentPoint {
    pub position: Vec2,          // Relative to base center
    pub rotation: f32,           // Rotation in radians
    pub size: Vec2,              // Size of the attachment area
    pub module_type: ModuleType,  // Type of module that can attach here
    pub occupied: bool,          // Whether this point is occupied
    pub attached_module: Option<Entity>, // Reference to attached module
}

/// Component for modules that can be attached to the base
#[derive(Component, Debug)]
pub struct AttachableModule {
    pub module_type: ModuleType,
    pub size: Vec2,             // Size of the module
    pub build_time: f32,         // Time to build/attach
    pub build_progress: f32,     // Current build progress (0.0-1.0)
}

/// Types of resources that can be stored or generated
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ResourceType {
    Basic,
    Wood,
    Stone,
    Iron,
    Copper,
    Alloy,
    Energy,
    Fuel,
    Ammunition,
    Research,
    Population,
}

/// Types of damage that can be dealt by weapons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DamageType {
    Kinetic,    // Standard physical damage
    Energy,     // Laser, plasma, etc.
    Explosive,  // Rockets, grenades
    Chemical,   // Acid, fire
    Sonic,      // Sound-based
    EMP,        // Anti-electronic
}

/// Types of utility effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum UtilityEffect {
    Repair,          // Repairs nearby friendly units/structures
    Cloak,          // Provides stealth to nearby units
    Jammer,         // Reduces enemy accuracy
    ShieldBoost,    // Temporarily increases shield strength
    SpeedBoost,     // Increases movement speed
    DamageAmp,      // Increases damage dealt
    Heal,           // Restores health over time
    Stun,           // Temporarily disables enemies
    Slow,           // Reduces enemy movement and attack speed
    Reveal,         // Reveals stealthed units
    Teleport,       // Short-range teleportation
    ResourceBoost,  // Increases resource gathering rate
}

// Default implementations for module creation
impl Default for BaseModule {
    fn default() -> Self {
        Self::new_movement_module(1.0, 0.8, 0.0)
    }
}

impl BaseModule {
    /// Create a new movement module
    pub fn new_movement_module(speed_mod: f32, efficiency: f32, terrain_penalty_reduction: f32) -> Self {
        Self {
            module_type: ModuleType::Movement {
                speed_modifier: speed_mod,
                efficiency,
                terrain_penalty_reduction,
            },
            health: 100.0,
            max_health: 100.0,
            power_consumption: 15.0 * (1.0 - efficiency).max(0.1), // More efficient = less power
            active: true,
            team: Team::Player,
        }
    }
    
    /// Create a new storage module
    pub fn new_storage_module(capacity: i32, resource_type: ResourceType, passive_gen: f32) -> Self {
        Self {
            module_type: ModuleType::Storage {
                capacity: capacity,
                resource_type: resource_type,
                passive_generation: passive_gen,
            },
            health: 120.0,
            max_health: 120.0,
            power_consumption: 5.0 + (passive_gen * 0.5), // More generation = more power
            active: true,
            team: Team::Player,
        }
    }
    
    /// Create a new defense module
    pub fn new_defense_module(armor: f32, shield: f32, recharge: f32, resistance: f32) -> Self {
        Self {
            module_type: ModuleType::Defense {
                armor_bonus: armor,
                shield_strength: shield,
                shield_recharge_rate: recharge,
                damage_resistance: resistance,
            },
            health: 150.0,
            max_health: 150.0,
            power_consumption: 20.0 + (shield * 0.01) + (recharge * 2.0),
            active: true,
            team: Team::Player,
        }
    }
    
    /// Create a new sensor module
    pub fn new_sensor_module(detection: f32, stealth: f32, vision: f32, cooldown: f32) -> Self {
        Self {
            module_type: ModuleType::Sensor {
                detection_radius: detection,
                stealth_detection: stealth,
                vision_range: vision,
                scan_cooldown: cooldown,
            },
            health: 80.0,
            max_health: 80.0,
            power_consumption: 25.0 * (1.0 + vision * 0.01),
            active: true,
            team: Team::Player,
        }
    }
    
    /// Create a new weapon module
    pub fn new_weapon_module(
        damage: f32,
        attack_speed: f32,
        range: f32,
        damage_type: DamageType,
        splash: f32,
        tracking: f32,
    ) -> Self {
        Self {
            module_type: ModuleType::Weapon {
                damage: damage,
                attack_speed: attack_speed,
                range: range,
                damage_type: damage_type,
                splash_radius: splash,
                tracking_speed: tracking,
            },
            health: 100.0,
            max_health: 100.0,
            power_consumption: 30.0 + (damage * attack_speed * 0.1),
            active: true,
            team: Team::Player,
        }
    }
    
    /// Create a new utility module
    pub fn new_utility_module(
        effect: UtilityEffect,
        strength: f32,
        aoe: f32,
        cooldown: f32,
    ) -> Self {
        Self {
            module_type: ModuleType::Utility {
                effect_type: effect,
                effect_strength: strength,
                area_of_effect: aoe,
                cooldown: cooldown,
            },
            health: 90.0,
            max_health: 90.0,
            power_consumption: match effect {
                UtilityEffect::Repair => 15.0 * strength,
                UtilityEffect::Cloak => 20.0 * aoe * 0.1,
                UtilityEffect::ShieldBoost => 25.0 * strength,
                _ => 15.0,
            },
            active: true,
            team: Team::Player,
        }
    }
}

impl AttachmentPoint {
    /// Create a new attachment point
    pub fn new(position: Vec2, rotation: f32, size: Vec2, module_type: ModuleType) -> Self {
        Self {
            position,
            rotation,
            size,
            module_type,
            occupied: false,
            attached_module: None,
        }
    }
    
    /// Check if a module can attach to this point
    pub fn can_attach(&self, module: &AttachableModule) -> bool {
        !self.occupied && self.matches_module_type(&module.module_type)
    }
    
    /// Check if this point accepts a specific module type
    pub fn matches_module_type(&self, module_type: &ModuleType) -> bool {
        use std::mem::discriminant;
        
        // Compare the discriminants of the two enums
        discriminant(&self.module_type) == discriminant(module_type)
    }
}

/// Plugin for base module systems
pub struct BaseModulePlugin;

impl Plugin for BaseModulePlugin {
    fn build(&self, app: &mut App) {
        // Register reflection types
        app.register_type::<BaseModule>()
            .register_type::<ModuleType>()
            // Add systems
            .add_systems(Update, (update_module_effects, handle_module_attachment));
    }
}

/// System to update active module effects
fn update_module_effects(
    mut bases: Query<(&Children, &mut MechanicalBase)>,
    modules: Query<&BaseModule>,
) {
    for (children, mut base) in bases.iter_mut() {
        // Reset base stats
        base.base_movement_speed = base.base_movement_speed; // Base speed
        
        // Apply module effects
        for &child in children.iter() {
            if let Ok(module) = modules.get(child) {
                if module.active {
                    match &module.module_type {
                        ModuleType::Movement { speed_modifier, .. } => {
                            base.effective_movement_speed = base.base_movement_speed * *speed_modifier;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// System to handle module attachment/detachment
pub fn handle_module_attachment(
    _commands: Commands,
    _attachment_points: Query<(&mut AttachmentPoint, &GlobalTransform)>,
    _modules: Query<(&mut AttachableModule, &Transform, Entity)>, 
    _mouse_buttons: Res<ButtonInput<MouseButton>>,
    _windows: Query<&Window>,
    _camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // Implementation will be added later
}

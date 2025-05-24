use bevy::prelude::*;
use strategy_forge::{
    components::{
        base_modules::{BaseModule, ModuleType, DamageType, ResourceType},
        unit::Team,
    },
};

/// Helper function to create a weapon module for testing
fn create_weapon_module() -> BaseModule {
    BaseModule {
        module_type: ModuleType::Weapon {
            damage: 10.0,
            damage_type: DamageType::Kinetic,
            range: 5.0,
            attack_speed: 1.0,
            splash_radius: 0.0,
            tracking_speed: 10.0,
        },
        health: 100.0,
        max_health: 100.0,
        power_consumption: 1.0,
        active: true,
        team: Team::Player,
    }
}

/// Helper function to create a storage module for testing
fn create_storage_module() -> BaseModule {
    BaseModule {
        module_type: ModuleType::Storage {
            capacity: 100,
            resource_type: ResourceType::Energy,
            passive_generation: 1.0,
        },
        health: 100.0,
        max_health: 100.0,
        power_consumption: 0.0,
        active: true,
        team: Team::Player,
    }
}

#[test]
fn test_weapon_module_creation() {
    // Test creating a weapon module with specific properties
    let module = create_weapon_module();
    
    // Verify the module was created with the right properties
    if let ModuleType::Weapon { 
        damage, 
        damage_type, 
        range, 
        attack_speed, 
        splash_radius, 
        tracking_speed 
    } = module.module_type {
        assert_eq!(damage, 10.0, "Weapon damage should be 10.0");
        assert_eq!(range, 5.0, "Weapon range should be 5.0");
        assert_eq!(attack_speed, 1.0, "Weapon attack speed should be 1.0");
        assert_eq!(tracking_speed, 10.0, "Tracking speed should be 10.0");
        assert_eq!(splash_radius, 0.0, "Splash radius should be 0.0");
        assert!(matches!(damage_type, DamageType::Kinetic), "Damage type should be Kinetic");
    } else {
        panic!("Module should be a weapon");
    }
    
    // Test base module properties
    assert_eq!(module.health, 100.0, "Module health should be 100.0");
    assert_eq!(module.max_health, 100.0, "Module max health should be 100.0");
    assert_eq!(module.power_consumption, 1.0, "Power consumption should be 1.0");
    assert!(module.active, "Module should be active");
    assert!(matches!(module.team, Team::Player), "Team should be Player");
}

#[test]
fn test_storage_module_creation() {
    // Test creating a storage module with specific properties
    let module = create_storage_module();
    
    // Verify the module was created with the right properties
    if let ModuleType::Storage { 
        capacity, 
        resource_type, 
        passive_generation 
    } = module.module_type {
        assert_eq!(capacity, 100, "Storage capacity should be 100");
        assert_eq!(passive_generation, 1.0, "Passive generation should be 1.0");
        assert!(matches!(resource_type, ResourceType::Energy), "Resource type should be Energy");
    } else {
        panic!("Module should be a storage module");
    }
    
    // Test base module properties
    assert_eq!(module.health, 100.0, "Module health should be 100.0");
    assert_eq!(module.max_health, 100.0, "Module max health should be 100.0");
    assert_eq!(module.power_consumption, 0.0, "Power consumption should be 0.0");
    assert!(module.active, "Module should be active");
    assert!(matches!(module.team, Team::Player), "Team should be Player");
}

#[test]
fn test_module_activation() {
    // Test module activation/deactivation
    let mut module = create_weapon_module();
    
    // Test initial state
    assert!(module.active, "Module should be active by default");
    
    // Test deactivation
    module.active = false;
    assert!(!module.active, "Module should be deactivated");
    
    // Test reactivation
    module.active = true;
    assert!(module.active, "Module should be reactivated");
}

#[test]
fn test_team_assignment() {
    // Test team assignment for modules
    let mut module = create_weapon_module();
    
    // Test initial team
    assert!(matches!(module.team, Team::Player), "Initial team should be Player");
    
    // Change team
    module.team = Team::Enemy;
    assert!(matches!(module.team, Team::Enemy), "Team should be changed to Enemy");
    
    // Change to neutral
    module.team = Team::Neutral;
    assert!(matches!(module.team, Team::Neutral), "Team should be changed to Neutral");
}

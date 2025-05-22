use bevy::prelude::*;

/// Component that marks an entity as controlled by AI
#[derive(Component, Debug)]
pub struct AIControlled {
    pub difficulty: AIDifficulty,
}

/// Different AI difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIDifficulty {
    Easy,
    Medium,
    Hard,
}

impl Default for AIControlled {
    fn default() -> Self {
        Self {
            difficulty: AIDifficulty::Medium,
        }
    }
}

/// Component for the AI player's base
#[derive(Component, Debug)]
pub struct AIBase;

/// Plugin for AI behavior
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            ai_controller
        );
        
        info!("AI Plugin initialized");
    }
}

/// System that controls AI behavior
fn ai_controller(
    time: Res<Time>,
    ai_bases: Query<(Entity, &Transform), With<AIBase>>,
) {
    // For now, just log that the AI is thinking
    if (time.elapsed_seconds() * 10.0) as u64 % 50 == 0 {
        for (entity, _transform) in ai_bases.iter() {
            debug!("AI is thinking for entity {:?}", entity);
        }
    }
    
    // Later we'll implement more complex AI behaviors here:
    // - Resource gathering
    // - Base movement toward strategic locations
    // - Unit production and management
    // - Combat tactics
}

use bevy::prelude::*;
use crate::components::unit::Unit;
use crate::utils::font_loader::{GameFonts, FontStyle, get_text_style};

/// Component for unit labels that display above units
#[derive(Component)]
pub struct UnitLabel {
    pub unit_entity: Entity,
}

/// Plugin that handles the creation and updating of unit labels
pub struct UnitLabelPlugin;

impl Plugin for UnitLabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_unit_labels,
            spawn_missing_labels,
        ));
    }
}

/// Spawn text labels for units that don't have them yet
fn spawn_missing_labels(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    units: Query<(Entity, &Unit), Without<UnitLabel>>,
    labels: Query<&UnitLabel>,
) {
    // Find units without labels
    for (unit_entity, unit) in units.iter() {
        // Check if a label already exists for this unit
        let has_label = labels.iter().any(|label| label.unit_entity == unit_entity);
        if !has_label {
            // Create a new text label for the unit
            let label_color = match unit.team {
                crate::components::unit::Team::Player => Color::srgb(0.2, 0.6, 1.0),
                crate::components::unit::Team::Enemy => Color::srgb(1.0, 0.3, 0.3),
                crate::components::unit::Team::Neutral => Color::srgb(0.8, 0.8, 0.8),
            };
            
            // Calculate health percentage
            let health_percentage = (unit.health / unit.max_health * 100.0).round();
            
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        format!("HP: {}%", health_percentage as i32),
                        get_text_style(&game_fonts, FontStyle::UnitLabel, label_color),
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.1)), // Position above unit
                    ..default()
                },
                UnitLabel { unit_entity },
            ));
        }
    }
}

/// Update the position and content of unit labels
fn update_unit_labels(
    mut labels: Query<(&mut Transform, &mut Text, &UnitLabel)>,
    units: Query<(&Transform, &Unit)>,
) {
    for (mut label_transform, mut text, label) in labels.iter_mut() {
        if let Ok((unit_transform, unit)) = units.get(label.unit_entity) {
            // Update position to follow the unit
            label_transform.translation.x = unit_transform.translation.x;
            label_transform.translation.y = unit_transform.translation.y + 15.0; // Position above unit
            
            // Update health display
            let health_percentage = (unit.health / unit.max_health * 100.0).round() as i32;
            if let Some(section) = text.sections.first_mut() {
                section.value = format!("HP: {}%", health_percentage);
            }
        }
    }
}

use bevy::prelude::*;

pub struct DebugPlugin;

const DEBUG_TOGGLE: bool = true;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if DEBUG_TOGGLE {
            app.add_systems(Update, print_position);
        }
    }
}


fn print_position(query: Query<(Entity, &Transform)>) {
    // Log the entity ID and the transform of each entity with a Transform
    for (entity, transform) in query.iter() {
        println!(
            "Entity {:?} is at position {:?},",
            entity, transform.translation
        );
    }
}


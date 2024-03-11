use crate::{bee::*, movement::Velocity};
use bevy::prelude::*;

pub struct DebugPlugin;

const DEBUG_TOGGLE: bool = true;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if DEBUG_TOGGLE {
            app.add_systems(
                PostUpdate,
                (
                    print_position,
                    debug_neighbors,
                    debug_separation,
                    debug_movement,
                ),
            );
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

fn debug_neighbors(query: Query<(&Bee, Entity)>) {
    for (bee, entity) in query.iter() {
        println!(
            "entity {1} has {0} neighbors",
            bee.nearby_bees.len(),
            entity.index()
        );
    }
}

fn debug_separation(query: Query<(&Bee, Entity)>) {
    for (bee, entity) in query.iter() {
        println!(
            "entity {1} has separation vector of {0} ",
            bee.separation,
            entity.index()
        );
    }
}

fn debug_movement(query: Query<(&Bee, Entity, &Velocity)>) {
    for (bee, entity, velocity) in query.iter() {
        println!(
            "A velocity of {0} was calculated for entity {1},\n using random noise,\n cohesion {2},\n separation {3},\n and alignment {4}",
            velocity.value,
            entity.index(),
            bee.cohesion,
            bee.separation,
            bee.alignment
        )
    }
}

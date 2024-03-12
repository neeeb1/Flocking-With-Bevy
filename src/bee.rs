use std::ops::Range;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

// Intialization constants
const INTIAL_SPEED: f32 = 25.0;
const SPAWN_COUNT: i32 = 1250;
const SPAWN_RANGE_X: Range<f32> = -350.0..350.0;
const SPAWN_RANGE_Z: Range<f32> = -200.0..200.0;

// Movement Constants
const VISION_RANGE: f32 = 10.0;
const SEPARATION_DISTANCE: f32 = 9.5;
const NOISE: f32 = 20.0;

// Derives a component Bee with...
// Vec of bees within VISION_RANGE by their transform and velocity
// cohesion, separation, and alignment targets
#[derive(Component, Debug)]
pub struct Bee {
    pub in_sight_bees: Vec<(Transform, Velocity)>,
    pub cohesion: Vec3,
    pub separation: Vec3,
    pub alignment: Vec3,
}

pub struct BeePlugin;

impl Plugin for BeePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_bees);
        app.add_systems(First, get_in_sight_bees);
        app.add_systems(
            PreUpdate,
            (coehsion, separation, alignment, calculate_movement),
        );
    }
}

// Spawn bees randomly throughout a the x and z range (big rectangle)
// Gives bees a random intial direction on the x / z axis with a noisy velocity and no acceleration
fn spawn_bees(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let mut spawned = 0;
    for _bee in 0..SPAWN_COUNT {
        let mut rng = rand::thread_rng();

        let rand_translation = Vec3::new(
            rng.gen_range(SPAWN_RANGE_X),
            0.,
            rng.gen_range(SPAWN_RANGE_Z),
        );

        let mut random_unit_vector = || {
            Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero()
        };
        let velocity = random_unit_vector() * INTIAL_SPEED;
        let acceleration = Vec3::ZERO;

        // Creates a moving object bundle (see movement.rs)
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(velocity),
                acceleration: Acceleration::new(acceleration),
                model: SceneBundle {
                    scene: scene_assets.bee.clone(),
                    transform: Transform::from_translation(rand_translation)
                        .looking_to(velocity, Vec3::Y),
                    ..default()
                },
            },
            // Tags the entity with Bee struct
            Bee {
                in_sight_bees: Vec::new(),
                cohesion: Vec3::ZERO,
                separation: Vec3::ZERO,
                alignment: Vec3::ZERO,
            },
        ));
        spawned += 1;
    }

    print!("{spawned} bees have been spawned!");
}

// Defines a function that takes two Queries...,
// and finds the bees in it's VISION_RANGE
fn get_in_sight_bees(
    mut mut_bees: Query<(Entity, &mut Bee, &Transform)>, // a mutable reference to Entities with that borrows a mut Bee and a Transform
    imut_bees: Query<(&Transform, &Velocity), With<Bee>>, // and an immutable reference to all entities with a Transform and Velocity (with the Bee componenent, but not borrowing it)
) {
    // Iterate over all the mutable bee references
    for (_entity, mut bee, transform) in mut_bees.iter_mut() {
        // Place holder Vec of tuples to set Vec bee.in_sight_bees later
        let mut vec_in_sight = Vec::<(Transform, Velocity)>::new();

        // Iterate over all the imuatable entities with bee
        for (other_transform, other_velocity) in imut_bees.iter() {
            // Calculates the distance between the two entities
            let distance = transform.translation.distance(other_transform.translation);

            // If within vision range, add to placeholder Vec
            if distance <= VISION_RANGE {
                vec_in_sight.push((
                    other_transform.to_owned(),
                    Velocity::new(other_velocity.value),
                ));
            }
        }

        // Updates bee with the placeholder Vec
        bee.in_sight_bees = vec_in_sight;
    }
}

/**
 * Cohesion is the tendency of individuals in a flock to move towards the center of mass of neighboring entities.
 * In a simulation, this behavior can be implemented by calculating the average position of in_sight entities
 *  and adjusting the velocity of each entity to move towards that average position.
 * - ChatGPT tbh
**/
fn coehsion(mut mut_bees: Query<(Entity, &mut Bee, &Transform)>) {
    for (_entity, mut bee, transform) in mut_bees.iter_mut() {
        let mut average_position = Vec3::ZERO;

        for (in_sight_transform, _) in &bee.in_sight_bees {
            let distance = transform
                .translation
                .distance(in_sight_transform.translation);

            if distance <= VISION_RANGE {
                average_position += in_sight_transform.translation;
            }
        }

        average_position /= bee.in_sight_bees.len() as f32;

        bee.cohesion = average_position;
    }
}

/**
 * Separation is the tendency of individuals to maintain a minimum distance from each other.
 * In a flocking simulation, separation can be implemented by calculating the direction away from in_sight entities
 * and adjusting the velocity to move in that direction.
 * - ChatGPT
**/
fn separation(mut mut_bees: Query<(Entity, &mut Bee, &Transform)>) {
    for (_entity, mut bee, transform) in mut_bees.iter_mut() {
        let mut separation_target = Vec3::ZERO;

        for (in_sight_transform, _) in &bee.in_sight_bees {
            let distance = transform
                .translation
                .distance(in_sight_transform.translation);

            if distance > 0.0 && distance < SEPARATION_DISTANCE {
                let direction = transform.translation - in_sight_transform.translation;
                separation_target += direction.normalize() * distance;
            }
        }
        bee.separation = separation_target;
    }
}

/**
 * Alignment is the tendency of individuals to align their velocity vectors with those of in_sight entities.
 * This behavior helps maintain the overall direction of the flock.
 * In a simulation, alignment can be implemented by calculating the average velocity of in_sight entities
 * and adjusting the velocity of each entity to match that average velocity.
 * - ChatGPT
**/
fn alignment(mut mut_bees: Query<(Entity, &mut Bee)>) {
    for (_entity, mut bee) in mut_bees.iter_mut() {
        let mut average_velocity = Vec3::ZERO;

        for (_, other_velocity) in &bee.in_sight_bees {
            average_velocity += other_velocity.value;
        }

        average_velocity /= bee.in_sight_bees.len() as f32;

        bee.alignment = average_velocity;
    }
}

fn calculate_movement(mut all_bees: Query<(Entity, &Bee, &mut Velocity, &Transform)>) {
    let mut rng = rand::thread_rng();

    for (_entity, bee, mut velocity, transform) in all_bees.iter_mut() {
        let random_velocity = Vec3::new(
            rng.gen_range(-NOISE..NOISE),
            0.,
            rng.gen_range(-NOISE..NOISE),
        );

        if bee.in_sight_bees.len() > 0 {
            let direction = (bee.cohesion + bee.separation + bee.alignment + random_velocity)
                - transform.translation;
            velocity.value.x += direction.x;
            velocity.value.z += direction.z;

            velocity.value = velocity.value.normalize() * INTIAL_SPEED;
        }
    }
}

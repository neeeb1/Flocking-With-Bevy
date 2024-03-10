use std::ops::Range;

use bevy::{prelude::*, scene::ron::de::Position};
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const INTIAL_SPEED: f32 = 25.0;
const SPAWN_RANGE_X: Range<f32> = -300.0..300.0;
const SPAWN_RANGE_Y: Range<f32> = -150.0..150.0;
const SPAWN_COUNT: i32 = 1000;
const NEARBY_RANGE: f32 = 25.0;

#[derive(Component, Debug)]
pub struct Bird {
    pub nearby_birds: i32,
    pub cohesion: Vec3,
    pub separation: Vec3,
    pub alignment: Vec3,
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_birds);
        app.add_systems(
            PreUpdate,
            (coehsion, separation, alignment, calculate_movement),
        );
    }
}

fn spawn_birds(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let mut spawned = 0;
    for _bird in 0..SPAWN_COUNT {
        let mut rng = rand::thread_rng();

        let rand_translation = Vec3::new(
            rng.gen_range(SPAWN_RANGE_X),
            0.,
            rng.gen_range(SPAWN_RANGE_Y),
        );

        let mut random_unit_vector = || {
            Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero()
        };
        let velocity = random_unit_vector() * INTIAL_SPEED;
        let acceleration = Vec3::ZERO; //Vec3::ONE * rng.gen_range(1.0..2.0);

        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(velocity),
                acceleration: Acceleration::new(acceleration),
                model: SceneBundle {
                    scene: scene_assets.bird.clone(),
                    transform: Transform::from_translation(rand_translation)
                        .looking_to(velocity, Vec3::Y),
                    ..default()
                },
            },
            // Tags the entity with Bird struct
            Bird {
                nearby_birds: 0,
                cohesion: Vec3::ZERO,
                separation: Vec3::ZERO,
                alignment: Vec3::ZERO,
            },
        ));
        spawned += 1;
    }

    //print!("{spawned} birds have been spawned!");
}

/** TODO: Find out how to iterate over birds in radius around a specific bird, for every bird
 *https://github.com/camsjams/rust-flock-of-boids/tree/main
 *
 * Cohesion is the tendency of individuals in a flock to move towards the center of mass of neighboring entities.
 * In a simulation, this behavior can be implemented by calculating the average position of nearby entities
 *  and adjusting the velocity of each entity to move towards that average position.
**/
fn coehsion(
    mut mut_birds: Query<(Entity, &mut Bird, &Transform)>,
    imut_birds: Query<&Transform, With<Bird>>,
) {
    for (entity, mut bird, transform) in mut_birds.iter_mut() {
        let mut sum_nearby = 0;
        let mut average_position = Vec3::ZERO;

        for nearby_transform in imut_birds.iter() {
            let distance = transform.translation.distance(nearby_transform.translation);

            if distance <= NEARBY_RANGE {
                sum_nearby += 1;
                average_position += nearby_transform.translation;
            }
        }

        //average_position -= transform.translation;
        //sum_nearby -= 1;

        average_position = average_position / sum_nearby as f32;

        bird.cohesion = average_position;
        bird.nearby_birds = sum_nearby;

        println!(
            "entity {1} has {0} neighbors",
            bird.nearby_birds,
            entity.index()
        );
    }
}

/**TODO: all of it
 *
 * Separation is the tendency of individuals to maintain a minimum distance from each other.
 * In a flocking simulation, separation can be implemented by calculating the direction away from nearby entities
 * and adjusting the velocity to move in that direction.
**/
fn separation() {}

/**TODO: all of it
 *
 *
 * Alignment is the tendency of individuals to align their velocity vectors with those of nearby entities.
 * This behavior helps maintain the overall direction of the flock.
 * In a simulation, alignment can be implemented by calculating the average velocity of nearby entities
 * and adjusting the velocity of each entity to match that average velocity.
**/
fn alignment() {}

// TODO: Combine cohesion, separation, and alignment to create a velocity vector
fn calculate_movement(mut all_birds: Query<(Entity, &Bird, &mut Velocity, &Transform)>) {
    for (entity, bird, mut velocity, transform) in all_birds.iter_mut() {
        if bird.nearby_birds > 0 {
            let direction = bird.cohesion - transform.translation;
            velocity.value.x += direction.x;
            velocity.value.z += direction.z;

            velocity.value = velocity.value.normalize() * INTIAL_SPEED;

/*             println!(
                "A velocity of {1} was calculated for entity {0}",
                entity.index(),
                velocity.value
            ) */
        }
    }
}

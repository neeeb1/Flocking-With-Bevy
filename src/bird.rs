use std::ops::Range;

use bevy::{ecs::query, prelude::*};
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const INTIAL_SPEED: f32 = 25.0;
const SPAWN_RANGE_X: Range<f32> = -150.0..150.0;
const SPAWN_RANGE_Y: Range<f32> = -100.0..100.0;
const SPAWN_COUNT: i32 = 200;

#[derive(Component, Debug)]
pub struct Bird;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_birds);
        app.add_systems(Update, (coehsion, separation, alignment));
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
        let acceleration = Vec3::ONE * rng.gen_range(1.0..2.0);

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
            Bird,
        ));
        spawned += 1;
    }

    print!("{spawned} birds have been spawned!");
}

//TODO: Find out how to iterate over birds in radius around a specific bird, for every bird
// https://github.com/camsjams/rust-flock-of-boids/tree/main
fn coehsion(mut all_birds: Query<(Entity, &Transform)>) {
    let nearby_bird = 0;
    let average_velocity = Vec3::ZERO;
    for (entity) in all_birds.iter_mut() {
        
    }
}

//TODO: all of it
fn separation() {}

//TODO: all of it
fn alignment() {}

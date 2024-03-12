

use bevy::prelude::*;

// Implements a Velocity componenent with Vec3 value and fn new, which takes a Vec3 and sets value
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

// Implements an Acceleration componenent with Vec3 value and fn new, which takes a Vec3 and sets value
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}


// Creates a MovingObjectBundle that holds a Velocity, Acceleration, and a SceneBundle model (for 3d assets)
#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}

// Implements a Plugin called MovementPlugin, which adds 3 functions to the Update schedule
// The Update schedule is executed every frame, after PreUpdate and before PostUpdate
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity, update_rotation));
    }
}

/* Function that updates velocity based on acceleration and time passed since last frame
final velocity = intial velocity + acceleration * time
                     - issac Newton, probably */
fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
        velocity.value.y = 0.0;
    }
}

/* Function that updates rotation of every entity with a Velocity and a Transform 
to point at direction of travel, but kinda lazily

Determines new rotation looking roughly at velocity vector, and lerps towards it
(but not by time, just on a fixed scale of 0.05) */

fn update_rotation(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let new_rotation = transform
            .looking_to(velocity.value, Vec3::new(0., 1., time.delta_seconds()))
            .rotation;

        transform.rotation = transform.rotation.lerp(new_rotation, 0.05);
    }
}

// Function that updates the position of every entity with a Transform and a Velocity
// Moves transform by velocity and time passed since last frame
// distance = velocity * time
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

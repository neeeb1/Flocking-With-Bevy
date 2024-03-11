// Simulate boids flocking behavior in 3d space using bevy
// (with bees)

mod asset_loader;
mod bee;
mod camera;
mod debug;
mod movement;

use asset_loader::AssetLoaderPlugin;
use bee::BeePlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
// use debug::DebugPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
    
        //bevy built ins
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 900.0,
        })
        //TODO: add window settigns in preparation of adding UI?
        .add_plugins(DefaultPlugins)

        // Custom plugins.
        .add_plugins(CameraPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(BeePlugin)
        .run()
}

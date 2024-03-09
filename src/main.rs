// Trying to simulate flocking behavior in 3d using bevy

mod camera;
mod debug;
mod movement;
mod asset_loader;
mod bird;

use bevy::prelude::*;
use camera::CameraPlugin;
// use debug::DebugPlugin;
use movement::MovementPlugin;
use asset_loader::AssetLoaderPlugin;
use bird::BirdPlugin;

fn main() {
    App::new()
    //bevy built ins
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 600.0,
    })
    .add_plugins(DefaultPlugins)
    // Custom plugins.
    .add_plugins(CameraPlugin)
    // .add_plugins(DebugPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(BirdPlugin)
    .run()
}
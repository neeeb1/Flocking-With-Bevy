// Simulate boids flocking behavior in 3d space using bevy
// (with bees)

// Sources:
// https://www.youtube.com/watch?v=QbUPfMXXQIY                      >> An excellent demonstration of flocking simulation
// https://bevyengine.org/learn/quick-start/introduction/           >> Bevy guide
// https://bevy-cheatbook.github.io/                                >> Bevy cheatbook
// chat.openai.com
// https://www.youtube.com/@ZymartuGames                            >> Bevy basics tutorial  
// https://www.poly.pizza/m/6ktZgxSVVn1                             >> Bee asset

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
    
        //bevy built-in plugins
        .insert_resource(ClearColor(Color::rgb(0.3, 0., 0.3)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 900.0,
        })

        //TODO: add window settigns in preparation of adding UI?
        .add_plugins(DefaultPlugins)

        // Custom plugins
        .add_plugins(CameraPlugin)

        //TODO: a better way of debugging
        // .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(BeePlugin)
        .run()
}

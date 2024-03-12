use bevy::prelude::*;

// Camera distance from origin on y-axis
// Increase this to zoom camera out
const CAMERA_DISTANCE: f32 = 450.0 ;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {

        // Creates a up y-axis by camera distance and pointed down ( looking at (0,0,0) )
        // And sets the rest of the bundle to default...
        transform: Transform::from_xyz(0., CAMERA_DISTANCE, 0.).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}

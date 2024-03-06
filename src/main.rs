mod camera;
mod scene;
mod player;
mod physics;

use camera::CameraPlugin;
use scene::ScenePlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            ScenePlugin,
            PlayerPlugin,
            PhysicsPlugin
        )).run();
}

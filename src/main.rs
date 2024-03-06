mod camera;
mod scene;
mod player;

use camera::CameraPlugin;
use scene::ScenePlugin;
use player::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            ScenePlugin,
            PlayerPlugin
        )).run();
}

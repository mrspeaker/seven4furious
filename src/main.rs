mod camera;
mod scene;

use camera::CameraPlugin;
use scene::ScenePlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            ScenePlugin
        )).run();
}

use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_scene);
    }
}

fn init_scene(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });
    commands.spawn(SceneBundle {
        scene: assets.load("scene.gltf#Scene0"),
        transform: Transform::from_xyz(0.,0.,-200.)
            .with_scale(Vec3::ONE * 2.0),
        ..default()
    });

}

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
    assets: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: assets.load("plane.glb#Scene0"),
        transform: Transform::from_xyz(0.,0.,100.).with_scale(Vec3::ONE * 1.0),
        ..default()
    });

     commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
     });

}

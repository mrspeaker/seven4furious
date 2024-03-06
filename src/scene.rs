use bevy::prelude::*;
use std::f32::consts::PI;

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
        transform: Transform::from_xyz(0.,-3.0,-200.)
            .with_scale(Vec3::ONE * 2.0),
        ..default()
    });

    for i in 1..3 {
        commands.spawn(SceneBundle {
        scene: assets.load("plane.glb#Scene0"),
        transform: Transform::from_xyz((i as f32) * 100.0 + 100.0,0.0,0.)
            .with_scale(Vec3::ONE * 1.0),
            ..default()});
        commands.spawn(SceneBundle {
        scene: assets.load("plane.glb#Scene0"),
        transform: Transform::from_xyz((i as f32) * 100.0 + 100.0,80.0,0.)
            .with_scale(Vec3::ONE * 1.0),
            ..default()});
        commands.spawn(SceneBundle {
        scene: assets.load("plane.glb#Scene0"),
        transform: Transform::from_xyz((i as f32) * 100.0 + 100.0,160.0,0.)
            .with_scale(Vec3::ONE * 1.0),
            ..default()});


    }

    commands.spawn(SceneBundle {
        scene: assets.load("car.glb#Scene0"),
        transform: Transform::from_xyz(0.,0.,-300.)
            .with_scale(Vec3::ONE * 1.0)
            .with_rotation(Quat::from_rotation_y(-PI/2.0)),
        ..default()
    });

}

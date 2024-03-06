use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_camera);
    }
}

fn init_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            camera: bevy::prelude::Camera {
                hdr: true,
                ..default()
            },
           transform: Transform::from_xyz(
                0.0,
                0.0,
                0.0
            ).looking_to(Vec3::Z, Vec3::Y),
            ..default()
        },
        Camera
    ));
}

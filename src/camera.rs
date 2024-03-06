use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

#[derive(Component)]
pub struct Camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_camera)
            .add_systems(Update, sync_camera);
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
                200.0,
                -400.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera
    ));
}

fn sync_camera(
    player: Query<&Transform, With<Player>>,
    mut q: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    let Ok(p) = player.get_single() else { return; };
    let Ok(mut t) = q.get_single_mut() else { return; };

    //t.translation = p.translation;
    //t.rotation = p.rotation;
    t.translation.y = (p.translation.z + 150.0).clamp(10.0, 500.0);
    t.look_at(Vec3::new(0.0, 0.0, p.translation.z.clamp(-50.0, 50.0)), Vec3::Y);
}

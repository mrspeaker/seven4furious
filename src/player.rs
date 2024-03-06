use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::physics::{Impulse, Torque};

use std::f32::consts::PI;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, (
            update_player,
        ));
    }
}

fn init_player(
    mut cmds: Commands,
    assets: Res<AssetServer>,
) {
    cmds.spawn((
        SceneBundle {
            scene: assets.load("plane.glb#Scene0"),
            transform: Transform::from_xyz(0.,0.5,0.)
                .with_scale(Vec3::ONE * 1.0)
                .with_rotation(Quat::from_rotation_y(-PI / 1.0)),
//                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player,
        Impulse::new(),
        Torque::new(),
    ));
}

fn update_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q: Query<(&Transform, &mut Impulse, &mut Torque), With<Player>>)
{
    let Ok((t, mut impulse, mut torque)) = q.get_single_mut() else {
        return;
    };
    let dt = time.delta_seconds();

    let mut rot = Vec3::ZERO;

    for event in mouse_events.read() {
        rot.x = -event.delta.y; // Pitch
        rot.y = -event.delta.x; // Yaw
    }

    let mut imp = Vec3::ZERO;
    let mut rot = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { imp += Vec3::from(t.forward()) }
    if keys.pressed(KeyCode::KeyS) { imp += Vec3::from(t.back()) }
    if keys.pressed(KeyCode::KeyA) { rot += Vec3::Y }
    if keys.pressed(KeyCode::KeyD) { rot -= Vec3::Y }
    if keys.pressed(KeyCode::KeyQ) { imp += Vec3::from(t.down()) }
    if keys.pressed(KeyCode::KeyE) { imp += Vec3::from(t.up()) }

    if imp.length() > 0.0 {
        //t.translation += imp * dt * 10.0;
        const SPEED: f32 = 2.0;
        impulse.add_force(imp.normalize() * SPEED * dt);
    }

    if rot.length() > 0.0 {
        const SPEED: f32 = 0.025;
        torque.add_force(rot * SPEED * dt);
    }


    if mouse_buttons.just_pressed(MouseButton::Left) {
        //
    }
}

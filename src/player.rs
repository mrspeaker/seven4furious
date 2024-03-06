use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

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
            transform: Transform::from_xyz(35.,30.,150.)
                .with_scale(Vec3::ONE * 1.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player,
    ));
}

fn update_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q: Query<&mut Transform, With<Player>>)
{
    let Ok(mut t) = q.get_single_mut() else {
        return;
    };
    let dt = time.delta_seconds();

    let mut rot = Vec3::ZERO;

    for event in mouse_events.read() {
        rot.x = -event.delta.y; // Pitch
        rot.y = -event.delta.x; // Yaw
    }

    let mut imp = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) { imp += Vec3::from(t.forward()); }
    if keys.pressed(KeyCode::KeyS) { imp += Vec3::from(t.back()); }
    if keys.pressed(KeyCode::KeyA) { imp += Vec3::from(t.left()); }
    if keys.pressed(KeyCode::KeyD) { imp += Vec3::from(t.right()); }
    if keys.pressed(KeyCode::KeyQ) { imp += Vec3::from(t.down());}
    if keys.pressed(KeyCode::KeyE) { imp += Vec3::from(t.up()); }

    if imp.length() > 0.0 {
        t.translation += imp * dt * 10.0;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        //
    }
}

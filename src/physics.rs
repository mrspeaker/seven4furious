use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            integrate
        );
    }
}

#[derive(Component)]
pub struct Bob;

#[derive(Component)]
pub struct Impulse {
    vel: Vec3,
    acc: Vec3
}

impl Impulse {
    pub fn new() -> Self {
        Self {
            vel: Vec3::ZERO,
            acc: Vec3::ZERO
        }
    }

    pub fn add_force(&mut self, v: Vec3) {
        self.acc += v;
    }

    pub fn speed(&self) -> f32 {
        return self.vel.length();
    }
}

#[derive(Component)]
pub struct Torque {
    vel: Vec3,
    acc: Vec3
}

impl Torque {
    pub fn new() -> Self {
        Self {
            vel: Vec3::ZERO,
            acc: Vec3::ZERO
        }
    }

    pub fn add_force(&mut self, v: Vec3) {
        self.acc += v;
    }
}

fn integrate(time: Res<Time>, mut q:Query<(&mut Transform, &mut Impulse, &mut Torque)>) {
    for (mut t, mut impulse, mut torque) in q.iter_mut() {

        //gravity
        impulse.acc.y -= 0.5 * time.delta_seconds();

        // Translation
        let acc = impulse.acc;
        impulse.vel += acc;
        impulse.vel *= 0.98; // Lol, friction
        impulse.acc = Vec3::ZERO;

        t.translation += impulse.vel;
        if t.translation.y < -0.1 {
            impulse.acc.y = -impulse.vel.y;
            impulse.vel.y = 0.0;
        }

        // Rotation
        let arot = torque.acc;
        torque.vel += arot;
        torque.vel *= 0.98; // Lol, friction
        torque.acc = Vec3::ZERO;

        //let a = (*t.forward()).dot(Vec3::Y).abs();
        t.rotate_local_x(torque.vel.x);
        //if a < 0.4 {
            //t.rotate_y(torque.vel.y); // Global y to eliminate roll
        //} else {
            t.rotate_local_y(torque.vel.y);
        //}

        t.rotate_local_z(torque.vel.z);

    }
}

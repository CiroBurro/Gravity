use crate::{
    components::{CaelestialBody, Velocity},
    physics::settings::GravitySettings,
};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GravitySettings::default())
            .add_systems(Update, apply_gravity)
            .add_systems(Update, update_positions);
    }
}

fn apply_gravity(
    mut query: Query<(&CaelestialBody, &mut Velocity, &Transform)>,
    settings: Res<GravitySettings>,
    time: Res<Time>,
) {
    let bodies: Vec<_> = query
        .iter()
        .map(|(body, vel, pos)| (body.mass, vel.0, pos.translation))
        .collect();
    let mut total_forces = vec![Vec3::ZERO; bodies.len()];

    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let body1 = bodies[i];
            let body2 = bodies[j];
            let delta = body2.2 - body1.2;
            let force =
                get_gravitation_force(body1.0, body2.0, settings.g, delta, settings.softening);
            total_forces[i] += force;
            total_forces[j] -= force;
        }
    }

    for ((body, mut vel, _), force) in query.iter_mut().zip(total_forces.iter()) {
        let acc = *force / body.mass;
        vel.0 += acc * time.delta_secs();
    }
}

fn update_positions(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.translation += vel.0 * time.delta_secs();
    }
}

fn get_gravitation_force(m1: f32, m2: f32, g: f32, delta: Vec3, softening: f32) -> Vec3 {
    let module: f32 = g * m1 * m2 / (delta.length().powf(2.0) + softening);

    delta.normalize() * module
}

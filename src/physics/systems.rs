use crate::{
    components::{CaelestialBody, Velocity},
    physics::settings::GravitySettings,
};
use bevy::prelude::*;

/// Bevy plugin that manages physics simulation systems.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GravitySettings::default())
            .add_systems(Update, apply_gravity)
            .add_systems(Update, update_positions);
    }
}

/// Calculates and applies gravitational forces between all celestial bodies.
///
/// This system implements N-body gravitational simulation using Newton's law of
/// universal gravitation. It calculates pairwise forces between all bodies and
/// updates their velocities based on the resulting accelerations.
///
/// # Algorithm
///
/// 1. Collect all body states (mass, velocity, position)
/// 2. Calculate pairwise forces for all body pairs (i < j optimization)
/// 3. Apply Newton's third law: forces are equal and opposite
/// 4. Update velocities using forward Euler integration: v' = v + (F/m)Δt
///
/// # Numerical Stability
///
/// Uses a softening factor in the force calculation to prevent singularities
/// when bodies approach very close distances, avoiding numerical instability.
///
/// # Formula
///
/// F = G × m₁ × m₂ / (r² + ε)
///
/// where ε is the softening factor
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

/// Updates positions of celestial bodies based on their velocities.
///
/// Implements forward Euler integration for position updates:
///
/// x' = x + v × Δt
///
/// This system runs after `apply_gravity` to ensure forces are applied before
/// position updates. The time step (Δt) is frame-dependent via Bevy's Time resource.
fn update_positions(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.translation += vel.0 * time.delta_secs();
    }
}

/// Calculates gravitational force between two masses.
///
/// Implements Newton's law of universal gravitation with softening:
///
/// F = G × m₁ × m₂ / (r² + ε) × r̂
///
/// where:
/// - G: gravitational constant
/// - m₁, m₂: masses of the two bodies
/// - r: distance between bodies
/// - ε: softening factor (prevents division by zero when r → 0)
/// - r̂: unit vector pointing from body 1 to body 2
///
/// # Parameters
///
/// - `m1`, `m2`: Masses of the two bodies
/// - `g`: Gravitational constant
/// - `delta`: Position vector from body 1 to body 2
/// - `softening`: Small value added to r² for numerical stability
///
/// # Returns
///
/// Force vector acting on body 1 (force on body 2 is negated per Newton's third law)
fn get_gravitation_force(m1: f32, m2: f32, g: f32, delta: Vec3, softening: f32) -> Vec3 {
    let module: f32 = g * m1 * m2 / (delta.length().powf(2.0) + softening);

    delta.normalize() * module
}

/// Calculates the gravitational potential at a given position for grid deformation.
///
/// This function computes a modified Newtonian gravitational potential used to
/// visualize spacetime curvature as a 2D grid deformation. The potential is calculated
/// using only the XZ plane projection (ignoring Y coordinate) to properly represent
/// the orbital plane.
///
/// # Modified Formula
///
/// φ = -Σ(G × M / r^α) × scale
///
/// where:
/// - G: gravitational constant
/// - M: mass of each celestial body
/// - r: 2D radial distance on the XZ plane
/// - α: decay exponent (0.6 by default, lower values extend the visual effect)
/// - scale: visualization scaling factor (0.01 by default)
///
/// # Why modified from standard 1/r?
///
/// The standard Newtonian potential (1/r) decays too rapidly for effective
/// visualization at solar system scales. Using r^0.6 instead creates a more
/// extended visual effect while maintaining the relative shape of curvature wells.
///
/// # Parameters
///
/// - `pos`: 3D position array [x, y, z] of the grid vertex
/// - `bodies`: Query containing all celestial bodies with mass and position
/// - `settings`: Gravity simulation settings (G constant, softening factor)
///
/// # Returns
///
/// The scaled gravitational potential value used to deform the grid's Y coordinate
pub fn get_gravitation_potential(
    pos: &[f32; 3],
    bodies: &Query<(&CaelestialBody, &Transform)>,
    settings: &Res<GravitySettings>,
) -> f32 {
    let mut potential = 0.0;
    // Project grid vertex onto XZ plane (orbital plane)
    let grid_xz = Vec2::new(pos[0], pos[2]);

    for (body, tf) in bodies.iter() {
        // Project body position onto XZ plane
        let body_xz = Vec2::new(tf.translation.x, tf.translation.z);
        let r = grid_xz.distance(body_xz);

        // Modified potential: 1/r^0.6 instead of 1/r for better visualization
        // Lower exponent = more extended effect across space
        potential -= body.mass * settings.g / (r.powf(0.6) + settings.softening);
    }

    // Scale down for visual range (adjust to taste)
    potential * 0.01
}

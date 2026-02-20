//! Core components for celestial bodies in the simulation.

use bevy::prelude::*;

/// Represents a celestial body with physical properties.
#[derive(Component)]
pub struct CaelestialBody {
    pub name: String,
    /// Mass in 10^24 kg (Earth = 5.97)
    pub mass: f32,
}

/// Velocity vector in km/s.
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Marker component for spacetime visualization grid.
#[derive(Component)]
pub struct GravityGrid;

/// Bundle containing all components needed for a celestial body.
#[derive(Bundle)]
pub struct CaelestialBodyBundle {
    pub body: CaelestialBody,
    pub velocity: Velocity,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}

impl Default for CaelestialBodyBundle {
    fn default() -> Self {
        CaelestialBodyBundle {
            body: CaelestialBody {
                name: "Sun".to_string(),
                mass: 1000.0,
            },
            velocity: Velocity(Vec3::default()),
            mesh: Mesh3d::default(),
            material: MeshMaterial3d::default(),
            transform: Transform::default(),
        }
    }
}

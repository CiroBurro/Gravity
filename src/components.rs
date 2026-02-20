use bevy::prelude::*;

#[derive(Component)]
pub struct CaelestialBody {
    pub name: String,
    pub mass: f32,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct GravityGrid;

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

//! Spacetime curvature visualization using a deformable 2D grid.
//!
//! This module implements a visual representation of gravitational fields by deforming
//! a wireframe grid based on the gravitational potential at each vertex. The grid provides
//! an intuitive visualization of how mass curves space, similar to the classic "rubber sheet"
//! analogy in general relativity.
//!
//! The deformation uses a modified Newtonian potential (r^0.6 instead of r^1.0) to extend
//! the visual effect across solar system distances, making orbital paths visually coherent
//! with the displayed curvature.

use bevy::{
    color::palettes::css::WHITE,
    mesh::{PlaneMeshBuilder, VertexAttributeValues},
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};

use crate::{
    components::{CaelestialBody, GravityGrid},
    physics::{settings::GravitySettings, systems::get_gravitation_potential},
};

/// Bevy plugin that manages the spacetime curvature visualization grid.
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WireframePlugin::default())
            .add_systems(Startup, spawn_grid)
            .add_systems(Update, update_grid);
    }
}

/// Spawns the spacetime curvature visualization grid at startup.
///
/// Creates a 2000×2000 unit wireframe plane positioned below the orbital plane
/// (at y = -5.0) with 80 subdivisions for smooth deformation visualization.
fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        GravityGrid,
        Mesh3d(
            meshes.add(
                PlaneMeshBuilder::from_size(Vec2::new(10_000.0, 10_000.0))
                    .subdivisions(100)
                    .build(),
            ),
        ),
        MeshMaterial3d(materials.add(Color::from(WHITE))),
        Wireframe,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

/// Updates the grid mesh vertices each frame to visualize spacetime curvature.
///
/// Deforms the grid by calculating the gravitational potential at each vertex
/// and adjusting its Y coordinate.
fn update_grid(
    bodies: Query<(&CaelestialBody, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    grid: Query<&Mesh3d, With<GravityGrid>>,
    settings: Res<GravitySettings>,
) {
    let mesh_handle = grid.single().unwrap();
    let mesh = meshes.get_mut(mesh_handle).unwrap();

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            let potential = get_gravitation_potential(&pos, &bodies, &settings);
            pos[1] = potential;
        }
    }
}

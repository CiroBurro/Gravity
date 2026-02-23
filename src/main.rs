//! Gravity simulation merging computer science with physics.
//!
//! This application simulates gravitational interactions between celestial bodies
//! using Newtonian physics and the Bevy game engine for visualization.

pub mod camera;
pub mod components;
pub mod grid;
pub mod physics;

use bevy::{
    color::palettes::css::{
        BLACK, BLUE, DARK_BLUE, LIGHT_BLUE, ORANGE, ORANGE_RED, RED, SANDY_BROWN, TAN, YELLOW,
    },
    prelude::*,
    window::CursorOptions,
};
use camera::CameraPlugin;
use components::*;
use grid::GridPlugin;
use physics::systems::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(CameraPlugin)
        .insert_resource(ClearColor(Color::from(BLACK).lighter(0.005)))
        .add_systems(Startup, setup)
        .run();
}

/// Initializes the simulation scene with camera, lighting, and celestial bodies.
///
/// Sets up the Solar System with the Sun and inner planets (Mercury, Venus, Earth, Mars, Jupiter).
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = false;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 600.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 50_000_000.0,
            range: 4000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0),
    ));

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Sun".to_string(),
            mass: 1989000.0,
        },
        velocity: Velocity(Vec3::ZERO),
        mesh: Mesh3d(meshes.add(Sphere::new(30.0))),
        material: MeshMaterial3d(materials.add(Color::from(RED))),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Mercury".to_string(),
            mass: 0.33,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -47.87)),
        mesh: Mesh3d(meshes.add(Sphere::new(6.0))),
        material: MeshMaterial3d(materials.add(Color::from(SANDY_BROWN))),
        transform: Transform::from_xyz(57.9, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Venus".to_string(),
            mass: 4.87,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -35.02)),
        mesh: Mesh3d(meshes.add(Sphere::new(9.0))),
        material: MeshMaterial3d(materials.add(Color::from(ORANGE))),
        transform: Transform::from_xyz(108.2, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Earth".to_string(),
            mass: 5.97,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -29.78)),
        mesh: Mesh3d(meshes.add(Sphere::new(10.0))),
        material: MeshMaterial3d(materials.add(Color::from(BLUE))),
        transform: Transform::from_xyz(149.6, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Mars".to_string(),
            mass: 0.64,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -24.07)),
        mesh: Mesh3d(meshes.add(Sphere::new(7.0))),
        material: MeshMaterial3d(materials.add(Color::from(ORANGE_RED))),
        transform: Transform::from_xyz(227.9, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Jupiter".to_string(),
            mass: 1898.0,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -13.07)),
        mesh: Mesh3d(meshes.add(Sphere::new(24.0))),
        material: MeshMaterial3d(materials.add(Color::from(YELLOW))),
        transform: Transform::from_xyz(778.5, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Saturn".to_string(),
            mass: 568.5,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -9.64)),
        mesh: Mesh3d(meshes.add(Sphere::new(20.0))),
        material: MeshMaterial3d(materials.add(Color::from(TAN))),
        transform: Transform::from_xyz(1432.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Neptune".to_string(),
            mass: 102.4,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -5.5)),
        mesh: Mesh3d(meshes.add(Sphere::new(16.0))),
        material: MeshMaterial3d(materials.add(Color::from(DARK_BLUE))),
        transform: Transform::from_xyz(4498.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(CaelestialBodyBundle {
        body: CaelestialBody {
            name: "Uranus".to_string(),
            mass: 86.82,
        },
        velocity: Velocity(Vec3::new(0.0, 0.0, -6.83)),
        mesh: Mesh3d(meshes.add(Sphere::new(16.0))),
        material: MeshMaterial3d(materials.add(Color::from(LIGHT_BLUE))),
        transform: Transform::from_xyz(2871.0, 0.0, 0.0),
        ..Default::default()
    });
}

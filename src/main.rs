pub mod components;
pub mod physics;

use bevy::{
    color::palettes::css::{BLUE, ORANGE, ORANGE_RED, RED, SANDY_BROWN, YELLOW},
    prelude::*,
};
use components::*;
use physics::systems::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1000.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            intensity: 10_000_000.0,
            range: 2000.0,
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
        mesh: Mesh3d(meshes.add(Sphere::new(15.0))),
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
        mesh: Mesh3d(meshes.add(Sphere::new(3.0))),
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
        mesh: Mesh3d(meshes.add(Sphere::new(4.5))),
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
        mesh: Mesh3d(meshes.add(Sphere::new(5.0))),
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
        mesh: Mesh3d(meshes.add(Sphere::new(3.5))),
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
        mesh: Mesh3d(meshes.add(Sphere::new(12.0))),
        material: MeshMaterial3d(materials.add(Color::from(YELLOW))),
        transform: Transform::from_xyz(778.5, 0.0, 0.0),
        ..Default::default()
    });
}

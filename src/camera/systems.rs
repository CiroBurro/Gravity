use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::camera::settings::CameraSettings;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Update, camera_movement)
            .add_systems(Update, camera_rotation);
    }
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    settings: Res<CameraSettings>,
) {
    let mut transform = query.single_mut().unwrap();
    let mut new_pos = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        new_pos += transform.forward().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        new_pos += transform.back().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        new_pos += transform.left().as_vec3();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        new_pos += transform.right().as_vec3();
    }
    if keyboard.pressed(KeyCode::Space) {
        new_pos += transform.up().as_vec3();
    }
    if keyboard.pressed(KeyCode::ControlLeft) {
        new_pos += transform.down().as_vec3();
    }

    // Speed modifier
    if keyboard.pressed(KeyCode::ShiftLeft) {
        transform.translation += new_pos * settings.speed * settings.speed_mod * time.delta_secs();
    } else {
        transform.translation += new_pos * settings.speed * time.delta_secs();
    }
}

fn camera_rotation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    settings: Res<CameraSettings>,
) {
    let mut transform = query.single_mut().unwrap();
    let delta = mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * settings.sensibility.x;
        let delta_pitch = -delta.y * settings.sensibility.y;

        let (mut yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw += delta_yaw;
        pitch += delta_pitch;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

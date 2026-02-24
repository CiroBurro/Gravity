use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::camera::settings::CameraSettings;

/// Bevy plugin that manages camera controls and movement.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Update, camera_movement)
            .add_systems(Update, camera_rotation);
    }
}

/// Handles keyboard-based camera movement in 3D space.
///
/// Reads WASD keys for horizontal movement, Space/Ctrl for vertical movement,
/// and applies frame-rate independent motion using the camera's local coordinate
/// system. Holding Shift applies a speed multiplier.
///
/// # Controls
///
/// - **W**: Move forward (camera's forward direction)
/// - **S**: Move backward
/// - **A**: Move left
/// - **D**: Move right
/// - **Space**: Move up
/// - **Left Ctrl**: Move down
/// - **Left Shift** (held): Apply speed multiplier from settings
///
/// # Algorithm
///
/// 1. Read keyboard input state for movement keys
/// 2. Accumulate movement vector in camera's local space
/// 3. Apply base speed or modified speed (if Shift held)
/// 4. Update camera translation using frame delta time
///
/// # Parameters
///
/// - `keyboard`: Bevy input resource for key states
/// - `time`: Bevy time resource for frame-independent movement
/// - `query`: Query for Camera3d transform (expects exactly one camera)
/// - `settings`: Camera configuration resource
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

/// Handles mouse-based camera rotation with pitch limiting.
///
/// Reads accumulated mouse motion and converts it to yaw/pitch rotation,
/// applying sensitivity scaling and clamping pitch to prevent gimbal lock.
///
/// # Algorithm
///
/// 1. Read mouse delta from Bevy's accumulated motion
/// 2. Scale delta by sensitivity settings (X for yaw, Y for pitch)
/// 3. Extract current yaw, pitch, roll from camera rotation
/// 4. Apply delta rotations to yaw and pitch
/// 5. Clamp pitch to ±(π/2 - 0.01) to prevent flipping
/// 6. Reconstruct rotation quaternion with clamped values
///
/// # Numerical Stability
///
/// Pitch is clamped to `FRAC_PI_2 - 0.01` (±89.4°) to prevent the camera
/// from reaching exactly vertical orientation, which would cause gimbal lock
/// and unpredictable rotation behavior.
///
/// # Parameters
///
/// - `mouse_motion`: Bevy resource containing accumulated mouse movement
/// - `query`: Query for Camera3d transform (expects exactly one camera)
/// - `settings`: Camera configuration resource with sensitivity values
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

//! Configuration parameters for camera behavior.
//!
use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraSettings {
    /// Base movement speed in world units per second.
    pub speed: f32,
    /// Speed multiplier when Shift key is held (default: 2.0).
    pub speed_mod: f32,
    /// Mouse sensitivity for camera rotation (X and Y axes).
    pub sensibility: Vec2,
}

impl Default for CameraSettings {
    /// Creates default camera settings.
    ///
    /// # Defaults
    ///
    /// - `speed`: 150.0 units/second
    /// - `speed_mod`: 2.0× multiplier with Shift
    /// - `sensibility`: 0.001 for both X and Y rotation
    fn default() -> Self {
        CameraSettings {
            speed: 150.0,
            speed_mod: 2.0,
            sensibility: Vec2::new(0.001, 0.001),
        }
    }
}

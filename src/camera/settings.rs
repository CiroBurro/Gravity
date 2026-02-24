use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraSettings {
    pub speed: f32,
    pub speed_mod: f32,
    pub sensibility: Vec2,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            speed: 150.0,
            speed_mod: 2.0,
            sensibility: Vec2::new(0.001, 0.001),
        }
    }
}

use bevy::prelude::*;

#[derive(Resource)]
pub struct GravitySettings {
    pub g: f32,
    pub softening: f32,
}

impl Default for GravitySettings {
    fn default() -> Self {
        GravitySettings {
            g: 0.1,
            softening: 0.1,
        }
    }
}

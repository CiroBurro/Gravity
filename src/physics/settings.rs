use bevy::prelude::*;

/// Configuration parameters for the gravity simulation.
#[derive(Resource)]
pub struct GravitySettings {
    /// Gravitational constant (scaled for simulation)
    pub g: f32,
    /// Softening factor to prevent singularities when bodies are very close
    pub softening: f32,
}

impl Default for GravitySettings {
    fn default() -> Self {
        GravitySettings {
            g: 0.07,
            softening: 0.1,
        }
    }
}

//! Physics simulation module implementing Newtonian gravitational dynamics.
//!
//! This module provides a complete N-body gravitational simulation using Newton's
//! law of universal gravitation. It handles force calculations, velocity updates,
//! and position integration for multiple celestial bodies.
//!
//! # Structure
//!
//! - [`settings`]: Configuration parameters (G constant, softening factor)
//! - [`systems`]: Core physics systems for force calculation and integration
//!
//! # Physics Model
//!
//! The simulation uses:
//! - **Newton's law of gravitation**: F = G × m₁ × m₂ / r²
//! - **Softening factor**: Prevents singularities at close distances
//! - **Forward Euler integration**: Simple first-order time integration
//! - **N-body simulation**: All bodies interact with all other bodies (O(n²))
//!
//! # Integration Method
//!
//! The simulation uses forward Euler integration with frame-dependent time steps:
//!
//! 1. Calculate all pairwise forces
//! 2. Update velocities: v' = v + (F/m)Δt
//! 3. Update positions: x' = x + vΔt
//!
//! This method is simple but sufficient for stable solar system simulation at
//! reasonable time scales.

pub mod settings;
pub mod systems;

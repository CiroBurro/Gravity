//! Camera control and movement system for the simulation.
//!
//! This module provides interactive camera controls for observing the gravitational
//! simulation. It handles keyboard-based movement (WASD + Space/Ctrl for vertical)
//! and mouse-based rotation with configurable speed and sensitivity.
//!
//! # Structure
//!
//! - [`settings`]: Configuration parameters (speed, sensitivity)
//! - [`systems`]: Camera movement and rotation systems
//!
//! # Controls
//!
//! - **W/A/S/D**: Forward/Left/Back/Right movement
//! - **Space**: Move up
//! - **Left Ctrl**: Move down
//! - **Left Shift**: Speed boost (2× multiplier)
//! - **Mouse**: Free-look rotation with pitch limiting
//!
//! # Integration
//!
//! The camera systems run in the `Update` schedule and operate on any entity
//! with a `Camera3d` component. Movement is frame-rate independent using Bevy's
//! `Time` resource for delta time.
//!
mod settings;
pub mod systems;

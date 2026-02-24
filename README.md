# Gravity Simulation

A real-time gravitational physics simulator that bridges computer science and physics, visualizing celestial body interactions using Newtonian mechanics.

## Overview

This project simulates gravitational forces between celestial bodies in 3D space using the Bevy game engine. It demonstrates how computational methods can accurately model physical phenomena, making abstract physics concepts tangible through interactive visualization.

## Features

- Real-time N-body gravitational simulation
- Newtonian physics with numerical stability optimizations
- 3D visualization of the Solar System
- Interactive camera controls with keyboard and mouse

## Project Structure

```
gravity/
├── src/
│   ├── main.rs           # Application entry point and scene setup
│   ├── components.rs     # ECS components for celestial bodies
│   ├── camera/
│   │   ├── mod.rs        # Camera module
│   │   ├── systems.rs    # Camera movement and rotation systems
│   │   └── settings.rs   # Camera configuration (speed, sensitivity)
│   ├── grid.rs           # Spacetime curvature visualization
│   └── physics/
│       ├── mod.rs        # Physics module
│       ├── systems.rs    # Gravity calculation and position updates
│       └── settings.rs   # Simulation parameters (G constant, softening)
├── Cargo.toml
└── README.md

## Building and Running

```bash
cargo run --release
```

## Camera Controls

- **W/A/S/D**: Move forward/left/backward/right
- **Space**: Move up
- **Left Ctrl**: Move down
- **Left Shift**: Speed boost (2× multiplier)
- **Mouse**: Free-look camera rotation

## How It Works

### Gravitational Forces

The simulation uses Newton's law of universal gravitation:

```
F = G * m1 * m2 / (r^2 + softening)
```

A softening factor is added to prevent numerical instabilities when bodies get very close. Forces are calculated between all pairs of bodies, and velocities are updated using forward Euler integration.

### Spacetime Curvature Visualization

The 2D grid beneath the solar system visualizes gravitational fields through mesh deformation, inspired by the classic "rubber sheet" analogy from general relativity. While Newton had no concept of spacetime curvature (that came 200+ years later with Einstein), his gravitational potential naturally provides a scalar field we can visualize:

```
φ = -Σ(G × M / r)
```

However, the standard Newtonian potential (1/r) decays too rapidly for effective visualization at solar system scales—orbital paths would appear disconnected from the visible curvature wells.

**Modified Formula for Better Visualization:**

```
φ = -Σ(G × M / r^0.6) × 0.01
```

By using `r^0.6` instead of `r^1.0`, we create a more extended visual effect that makes planetary orbits visually coherent with the displayed curvature. This modification doesn't affect the physics simulation—it's purely a visualization enhancement that helps convey how gravity "reaches out" across space to guide orbital motion.

The grid deforms in real-time based on the positions of all celestial bodies, creating gravitational "wells" that deepen near massive objects like the Sun and Jupiter.

## TODO

- [x] Add remaining planets (Saturn, Uranus, Neptune) and adjust camera positioning
- [X] Implement 2D grid visualization of spacetime curvature
- [X] Add movements to the camera
- [ ] Add runtime parameter adjustment menu (advanced feature)

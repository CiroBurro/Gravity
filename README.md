# Gravity Simulation

A real-time gravitational physics simulator that bridges computer science and physics, visualizing celestial body interactions using Newtonian mechanics.

## Overview

This project simulates gravitational forces between celestial bodies in 3D space using the Bevy game engine. It demonstrates how computational methods can accurately model physical phenomena, making abstract physics concepts tangible through interactive visualization.

## Features

- Real-time N-body gravitational simulation
- Newtonian physics with numerical stability optimizations
- 3D visualization of the Solar System
- Currently includes: Sun, Mercury, Venus, Earth, Mars, Jupiter

## Project Structure

```
gravity/
├── src/
│   ├── main.rs           # Application entry point and scene setup
│   ├── components.rs     # ECS components for celestial bodies
│   └── physics/
│       ├── mod.rs        # Physics module
│       ├── systems.rs    # Gravity calculation and position updates
│       └── settings.rs   # Simulation parameters (G constant, softening)
├── Cargo.toml
└── README.md
```

## Building and Running

```bash
cargo run --release
```

## How It Works

The simulation uses Newton's law of universal gravitation:

```
F = G * m1 * m2 / (r^2 + softening)
```

A softening factor is added to prevent numerical instabilities when bodies get very close. Forces are calculated between all pairs of bodies, and velocities are updated using forward Euler integration.

## TODO

- [x] Add remaining planets (Saturn, Uranus, Neptune) and adjust camera positioning
- [X] Implement 2D grid visualization of spacetime curvature
- [ ] Add movements to the camera
- [ ] Add runtime parameter adjustment menu (advanced feature)

## License

This project is open source and available under the MIT License.

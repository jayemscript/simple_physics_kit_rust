# Simple Physics Kit for Rust

A lightweight, open-source physics calculation library built entirely in Rust using the standard library.

Simple Physics Kit aims to provide reusable mathematical functions for basic physics calculations, scientific applications, educational projects, and future physics simulations.

## Features

The library is currently in early development.

Initial functionality focuses on classical mechanics and fundamental kinematics, including:

- Free-fall calculations
- Projectile motion
- Velocity calculations
- Acceleration calculations
- Mathematical validation and error handling using Rust's `Result` type

Features are implemented incrementally according to the development roadmap.

## Installation

Clone the repository:

```bash
git clone https://github.com/<your-username>/simple-physics-kit-rust.git
cd simple-physics-kit-rust
```

Build the library:

```bash
cargo build
```

Run the available tests:

```bash
cargo test
```

Generate the API documentation:

```bash
cargo doc --open
```

The library currently uses only the Rust standard library and requires no external dependencies.

## Usage

Example using the free-fall module:

```rust
use simple_physics_kit_rust::free_fall::{
    time_to_ground,
    impact_velocity,
};

fn main() {
    let height = 45.0;
    let gravity = 9.8;

    let time = time_to_ground(height, gravity).unwrap();
    let velocity = impact_velocity(height, gravity).unwrap();

    println!("Time to ground: {:.2} seconds", time);
    println!("Impact velocity: {:.2} m/s", velocity);
}
```

This example assumes an object is dropped from rest with constant gravitational acceleration and no air resistance.

## Development Roadmap

| Version | Planned Features |
|---|---|
| **v0.1.0** | Free fall, projectile motion, velocity, acceleration |
| **v0.2.0** | Newton's laws, force, mass, momentum |
| **v0.3.0** | Kinetic energy, potential energy, work |
| **v0.4.0** | 2D vectors, position, velocity components |
| **v0.5.0** | Gravitational interaction between two objects, orbital calculations |
| **v1.0.0** | Documented, tested, stable physics API |

Version numbers represent development goals and may change as the project evolves.

## Project Goals

- Provide simple and reusable physics calculation APIs.
- Maintain accurate mathematical implementations.
- Use only the Rust standard library.
- Follow idiomatic Rust programming practices.
- Implement input validation and meaningful error handling.
- Provide unit tests and documentation for public functions.
- Support future development of physics simulations and scientific tools.

## Contributing

Contributions, bug reports, and suggestions are welcome.

The project is in early development, and its public APIs may change before version 1.0.0.

## License

See the [LICENSE](LICENSE) file for licensing information.
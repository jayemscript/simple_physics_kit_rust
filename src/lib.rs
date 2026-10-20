//! # Simple Physics Kit
//!
//! A lightweight, dependency-free physics calculation
//! library written in Rust.
//!
//! ## Modules
//!
//! - `free_fall`: Free-fall physics
//! - `velocity`: One-dimensional velocity
//! - `projectile_motion`: Horizontal projectile motion
//! - `acceleration`: One-dimensional acceleration
//!
//! All calculations use SI units.

pub mod acceleration;
pub mod free_fall;
pub mod projectile_motion;
pub mod velocity;

// Free fall API
pub use free_fall::{impact_velocity, time_to_ground};

// Velocity API
pub use velocity::{average_velocity, displacement, final_velocity};

// Projectile motion API
pub use projectile_motion::{
    final_speed, final_vertical_velocity, horizontal_distance, time_of_flight,
};

// Acceleration API
pub use acceleration::{
    average_acceleration, displacement as accelerated_displacement,
    final_velocity as accelerated_final_velocity,
};

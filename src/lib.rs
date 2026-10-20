//! # Simple Physics Kit
//!
//! A lightweight physics calculation library written in Rust.
//!
//! Uses only the Rust standard library.
//!
//! ## Available Modules
//!
//! - `free_fall` - Free-fall calculations.
//! - `velocity` - One-dimensional velocity calculations.
//! - `projectile_motion` - Horizontal projectile motion.

pub mod free_fall;
pub mod velocity;
pub mod projectile_motion;

// Free fall
pub use free_fall::{
    impact_velocity,
    time_to_ground,
};

// Velocity
pub use velocity::{
    average_velocity,
    displacement,
    final_velocity,
};

// Projectile motion
pub use projectile_motion::{
    final_speed,
    final_vertical_velocity,
    horizontal_distance,
    time_of_flight,
};
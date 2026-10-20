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
//!
//! ## Example
//!
//! ```
//! use simple_physics_kit_rust::velocity::average_velocity;
//!
//! let result = average_velocity(100.0, 5.0).unwrap();
//!
//! assert_eq!(result, 20.0);
//! ```

pub mod free_fall;
pub mod velocity;

// Re-export free-fall functions.
pub use free_fall::{
    impact_velocity,
    time_to_ground,
};

// Re-export velocity functions.
pub use velocity::{
    average_velocity,
    displacement,
    final_velocity,
};
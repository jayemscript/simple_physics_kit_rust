//! # Simple Physics Kit
//!
//! A lightweight physics calculation library written in Rust.
//!
//! Uses only the Rust standard library.
//!
//! ## Available Modules
//!
//! - `free_fall` - Free-fall physics calculations.
//!
//! ## Example
//!
//! ```
//! use simple_physics_kit_rust::free_fall::{
//!     time_to_ground,
//!     impact_velocity,
//! };
//!
//! let time = time_to_ground(45.0, 9.8).unwrap();
//! let velocity = impact_velocity(45.0, 9.8).unwrap();
//!
//! assert!((time - 3.03).abs() < 0.01);
//! assert!((velocity - 29.70).abs() < 0.01);
//! ```

// Public modules
pub mod free_fall;

// Re-export public functions for convenient access
pub use free_fall::{
    time_to_ground,
    impact_velocity,
};
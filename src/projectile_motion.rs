//! Horizontal projectile motion calculations.
//!
//! Assumptions:
//! - The projectile is launched horizontally.
//! - Initial vertical velocity is zero.
//! - Gravitational acceleration is constant.
//! - Air resistance is ignored.
//!
//! Units:
//! - Height: meters
//! - Horizontal velocity: m/s
//! - Gravity: m/s²
//! - Time: seconds

use crate::free_fall;

/// Calculates the time of flight for a horizontal launch.
///
/// Returns time in seconds.
pub fn time_of_flight(
    height: f64,
    gravity: f64,
) -> Result<f64, &'static str> {
    let time = free_fall::time_to_ground(height, gravity)?;

    if !time.is_finite() {
        return Err("Calculated time is not finite");
    }

    Ok(time)
}

/// Calculates horizontal displacement until impact.
///
/// Horizontal velocity may be negative to represent
/// motion in the opposite direction.
///
/// Returns displacement in meters.
pub fn horizontal_distance(
    height: f64,
    horizontal_velocity: f64,
    gravity: f64,
) -> Result<f64, &'static str> {
    if !horizontal_velocity.is_finite() {
        return Err("Horizontal velocity must be finite");
    }

    let time = time_of_flight(height, gravity)?;
    let distance = horizontal_velocity * time;

    if !distance.is_finite() {
        return Err("Calculated distance is not finite");
    }

    Ok(distance)
}

/// Calculates vertical velocity just before impact.
///
/// Downward is considered positive.
///
/// Returns velocity in m/s.
pub fn final_vertical_velocity(
    height: f64,
    gravity: f64,
) -> Result<f64, &'static str> {
    let time = time_of_flight(height, gravity)?;
    let velocity = gravity * time;

    if !velocity.is_finite() {
        return Err("Calculated vertical velocity is not finite");
    }

    Ok(velocity)
}

/// Calculates the total speed just before impact.
///
/// Combines horizontal and vertical velocity components
/// using the Pythagorean theorem.
///
/// Returns speed in m/s.
pub fn final_speed(
    height: f64,
    horizontal_velocity: f64,
    gravity: f64,
) -> Result<f64, &'static str> {
    if !horizontal_velocity.is_finite() {
        return Err("Horizontal velocity must be finite");
    }

    let vertical = final_vertical_velocity(height, gravity)?;
    let speed = horizontal_velocity.hypot(vertical);

    if !speed.is_finite() {
        return Err("Calculated final speed is not finite");
    }

    Ok(speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_of_flight() {
        let result = time_of_flight(60.0, 9.8).unwrap();

        assert!((result - 3.4993).abs() < 0.001);
    }

    #[test]
    fn test_horizontal_distance() {
        let result = horizontal_distance(60.0, 15.0, 9.8).unwrap();

        assert!((result - 52.4891).abs() < 0.001);
    }

    #[test]
    fn test_final_vertical_velocity() {
        let result = final_vertical_velocity(60.0, 9.8).unwrap();

        assert!((result - 34.2929).abs() < 0.001);
    }

    #[test]
    fn test_final_speed() {
        let result = final_speed(60.0, 15.0, 9.8).unwrap();

        assert!((result - 37.4300).abs() < 0.01);
    }

    #[test]
    fn test_negative_height() {
        assert!(time_of_flight(-60.0, 9.8).is_err());
    }

    #[test]
    fn test_zero_gravity() {
        assert!(time_of_flight(60.0, 0.0).is_err());
    }

    #[test]
    fn test_nan_velocity() {
        assert!(
            horizontal_distance(60.0, f64::NAN, 9.8).is_err()
        );
    }

    #[test]
    fn test_negative_horizontal_velocity() {
        let result =
            horizontal_distance(60.0, -15.0, 9.8).unwrap();

        assert!((result + 52.4891).abs() < 0.001);
    }

    #[test]
    fn test_infinite_input() {
        assert!(
            final_speed(60.0, f64::INFINITY, 9.8).is_err()
        );
    }

    #[test]
    fn test_zero_height() {
        let result = time_of_flight(0.0, 9.8).unwrap();

        assert_eq!(result, 0.0);
    }
}
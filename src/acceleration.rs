//! One-dimensional acceleration calculations.
//!
//! Uses SI units:
//! - Velocity: m/s
//! - Acceleration: m/s²
//! - Time: seconds
//! - Displacement: meters
//!
//! Constant acceleration is assumed where applicable.

/// Calculates average acceleration.
///
/// initial_velocity: m/s
/// final_velocity: m/s
/// time: seconds
///
/// Returns acceleration in m/s².
pub fn average_acceleration(
    initial_velocity: f64,
    final_velocity: f64,
    time: f64,
) -> Result<f64, &'static str> {
    if !initial_velocity.is_finite() || !final_velocity.is_finite() || !time.is_finite() {
        return Err("All inputs must be finite numbers");
    }

    if time <= 0.0 {
        return Err("Time must be greater than zero");
    }

    let acceleration = (final_velocity - initial_velocity) / time;

    if !acceleration.is_finite() {
        return Err("Calculated acceleration is not finite");
    }

    Ok(acceleration)
}

/// Calculates final velocity under constant acceleration.
///
/// Returns velocity in m/s.
pub fn final_velocity(
    initial_velocity: f64,
    acceleration: f64,
    time: f64,
) -> Result<f64, &'static str> {
    if !initial_velocity.is_finite() || !acceleration.is_finite() || !time.is_finite() {
        return Err("All inputs must be finite numbers");
    }

    if time < 0.0 {
        return Err("Time cannot be negative");
    }

    let velocity = initial_velocity + acceleration * time;

    if !velocity.is_finite() {
        return Err("Calculated velocity is not finite");
    }

    Ok(velocity)
}

/// Calculates displacement under constant acceleration.
///
/// Returns displacement in meters.
pub fn displacement(
    initial_velocity: f64,
    acceleration: f64,
    time: f64,
) -> Result<f64, &'static str> {
    if !initial_velocity.is_finite() || !acceleration.is_finite() || !time.is_finite() {
        return Err("All inputs must be finite numbers");
    }

    if time < 0.0 {
        return Err("Time cannot be negative");
    }

    let distance = initial_velocity * time + 0.5 * acceleration * time.powi(2);

    if !distance.is_finite() {
        return Err("Calculated displacement is not finite");
    }

    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_acceleration() {
        let result = average_acceleration(10.0, 30.0, 5.0).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_negative_acceleration() {
        let result = average_acceleration(30.0, 10.0, 5.0).unwrap();
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_final_velocity() {
        let result = final_velocity(10.0, 4.0, 5.0).unwrap();
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_displacement() {
        let result = displacement(10.0, 4.0, 5.0).unwrap();
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_zero_time_error() {
        assert!(average_acceleration(10.0, 30.0, 0.0).is_err());
    }

    #[test]
    fn test_negative_time_error() {
        assert!(displacement(10.0, 4.0, -5.0).is_err());
    }

    #[test]
    fn test_nan_error() {
        assert!(final_velocity(f64::NAN, 4.0, 5.0).is_err());
    }

    #[test]
    fn test_infinite_input_error() {
        assert!(average_acceleration(10.0, f64::INFINITY, 5.0).is_err());
    }

    #[test]
    fn test_zero_acceleration() {
        let result = displacement(10.0, 0.0, 5.0).unwrap();
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_result_overflow() {
        assert!(displacement(f64::MAX, f64::MAX, 10.0).is_err());
    }
}

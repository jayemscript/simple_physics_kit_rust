//! Velocity calculations for one-dimensional motion.
//!
//! All functions use SI units and return an error
//! when the input values are invalid.

/// Calculates average velocity from displacement and time.
///
/// displacement: meters (signed)
/// time: seconds
///
/// Returns velocity in m/s.
pub fn average_velocity(displacement: f64, time: f64) -> Result<f64, &'static str> {
    if !displacement.is_finite() {
        return Err("Displacement must be finite");
    }

    if !time.is_finite() || time <= 0.0 {
        return Err("Time must be finite and greater than zero");
    }

    let velocity = displacement / time;

    if !velocity.is_finite() {
        return Err("Calculated velocity is not finite");
    }

    Ok(velocity)
}

/// Calculates final velocity under constant acceleration.
///
/// initial_velocity: m/s
/// acceleration: m/s²
/// time: seconds
///
/// Returns final velocity in m/s.
pub fn final_velocity(
    initial_velocity: f64,
    acceleration: f64,
    time: f64,
) -> Result<f64, &'static str> {
    if !initial_velocity.is_finite() || !acceleration.is_finite() || !time.is_finite() {
        return Err("Inputs must be finite numbers");
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

/// Calculates displacement at constant velocity.
///
/// velocity: m/s
/// time: seconds
///
/// Returns displacement in meters.
pub fn displacement(velocity: f64, time: f64) -> Result<f64, &'static str> {
    if !velocity.is_finite() || !time.is_finite() {
        return Err("Inputs must be finite numbers");
    }

    if time < 0.0 {
        return Err("Time cannot be negative");
    }

    let result = velocity * time;

    if !result.is_finite() {
        return Err("Calculated displacement is not finite");
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_velocity() {
        let result = average_velocity(100.0, 5.0).unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_final_velocity() {
        let result = final_velocity(10.0, 2.0, 5.0).unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_displacement() {
        let result = displacement(20.0, 5.0).unwrap();
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_zero_time_error() {
        assert!(average_velocity(100.0, 0.0).is_err());
    }

    #[test]
    fn test_negative_time_error() {
        assert!(final_velocity(10.0, 2.0, -5.0).is_err());
    }

    #[test]
    fn test_nan_error() {
        assert!(displacement(f64::NAN, 5.0).is_err());
    }

    #[test]
    fn test_negative_velocity() {
        let result = average_velocity(-100.0, 5.0).unwrap();
        assert_eq!(result, -20.0);
    }

    #[test]
    fn test_overflow_error() {
        assert!(displacement(f64::MAX, 2.0).is_err());
    }
}

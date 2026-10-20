/// Calculates the time for an object dropped from rest
/// to reach the ground.
///
/// height: meters
/// gravity: m/s²
///
/// Returns time in seconds.
pub fn time_to_ground(height: f64, gravity: f64) -> Result<f64, &'static str> {
    if !height.is_finite() || height < 0.0 {
        return Err("Height must be a finite, non-negative number");
    }

    if !gravity.is_finite() || gravity <= 0.0 {
        return Err("Gravity must be a finite, positive number");
    }

    Ok(((2.0 * height) / gravity).sqrt())
}

/// Calculates the downward impact velocity of an object
/// dropped from rest.
///
/// Returns velocity in m/s, with downward positive.
pub fn impact_velocity(height: f64, gravity: f64) -> Result<f64, &'static str> {
    let time = time_to_ground(height, gravity)?;

    let velocity = gravity * time;

    Ok(velocity)
}
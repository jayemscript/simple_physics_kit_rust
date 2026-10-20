use simple_physics_kit_rust::acceleration::{average_acceleration, displacement, final_velocity};

fn main() -> Result<(), &'static str> {
    println!("=== Simple Physics Kit: Acceleration ===");

    let initial_velocity = 10.0;
    let final_velocity_value = 30.0;
    let time = 5.0;

    // Calculate acceleration from velocity change
    let acceleration = average_acceleration(initial_velocity, final_velocity_value, time)?;

    // Calculate final velocity
    let velocity = final_velocity(initial_velocity, acceleration, time)?;

    // Calculate displacement
    let distance = displacement(initial_velocity, acceleration, time)?;

    println!("Acceleration: {:.2} m/s²", acceleration);
    println!("Final velocity: {:.2} m/s", velocity);
    println!("Displacement: {:.2} meters", distance);

    Ok(())
}

use simple_physics_kit_rust::velocity::{average_velocity, displacement, final_velocity};

fn main() -> Result<(), &'static str> {
    println!("=== Simple Physics Kit: Velocity ===");

    // Average velocity
    let velocity = average_velocity(100.0, 5.0)?;
    println!("Average velocity: {:.2} m/s", velocity);

    // Final velocity under constant acceleration
    let final_v = final_velocity(10.0, 2.0, 5.0)?;
    println!("Final velocity: {:.2} m/s", final_v);

    // Displacement at constant velocity
    let distance = displacement(20.0, 5.0)?;
    println!("Displacement: {:.2} meters", distance);

    Ok(())
}

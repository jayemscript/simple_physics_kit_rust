use simple_physics_kit_rust::projectile_motion::{
    final_speed, final_vertical_velocity, horizontal_distance, time_of_flight,
};

fn main() -> Result<(), &'static str> {
    println!("=== Projectile Motion Calculator ===");

    let height = 60.0;
    let horizontal_velocity = 15.0;
    let gravity = 9.8;

    let time = time_of_flight(height, gravity)?;

    let distance = horizontal_distance(height, horizontal_velocity, gravity)?;

    let vertical = final_vertical_velocity(height, gravity)?;

    let speed = final_speed(height, horizontal_velocity, gravity)?;

    println!("Time of flight: {:.2} seconds", time);
    println!("Horizontal distance: {:.2} meters", distance);
    println!("Final vertical velocity: {:.2} m/s", vertical);
    println!("Final speed: {:.2} m/s", speed);

    Ok(())
}

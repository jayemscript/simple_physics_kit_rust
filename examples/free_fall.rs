use simple_physics_kit_rust::free_fall::{impact_velocity, time_to_ground};

fn main() {
    let time = time_to_ground(45.0, 9.8).unwrap();
    let velocity = impact_velocity(45.0, 9.8).unwrap();

    println!("Time: {:.2} seconds", time);
    println!("Impact velocity: {:.2} m/s", velocity);
}

use simple_physics_kit_rust::free_fall::{
    time_to_ground,
    impact_velocity,
};

fn main() {
    let time = time_to_ground(45.0, 9.8).unwrap();
    let velocity = impact_velocity(45.0, 9.8).unwrap();

    println!("Time: {:.2} seconds", time);
    println!("Impact velocity: {:.2} m/s", velocity);
}
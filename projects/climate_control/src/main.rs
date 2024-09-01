// src/main.rs
mod climate;
mod simulation;

use climate::ClimateControlSystem;
use simulation::run_simulation;

fn main() {
    let initial_cabin_temperature = 20.0;
    let external_temperature = 15.0;

    let mut system = ClimateControlSystem::new(initial_cabin_temperature, external_temperature);

    // Run the simulation
    run_simulation(&mut system);
}

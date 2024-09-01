// src/simulation.rs
use crate::climate::ClimateControlSystem;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

pub fn run_simulation(system: &mut ClimateControlSystem) {
    loop {
        println!("\n--- Simulating Climate Control System ---");
        println!("Current cabin temperature: {:.1}°C", system.current_temperature);
        println!("Desired cabin temperature: {:.1}°C", system.desired_temperature);
        println!("External temperature: {:.1}°C", system.external_temperature);

        // Adjust cabin temperature
        system.adjust_temperature();

        // Simulate changes in external conditions every few iterations
        if rand::thread_rng().gen_bool(0.2) {
            system.simulate_external_conditions();
        }

        // Wait for a short period to simulate real-time adjustments
        sleep(Duration::from_secs(1));

        // End the loop if the desired temperature is reached
        if (system.current_temperature - system.desired_temperature).abs() < 0.1 {
            println!("System stabilized at desired temperature.");
            break;
        }
    }
}

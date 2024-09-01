// src/climate.rs
use rand::Rng;

pub struct ClimateControlSystem {
    pub current_temperature: f32,
    pub desired_temperature: f32,
    pub external_temperature: f32,
}

impl ClimateControlSystem {
    pub fn new(initial_temperature: f32, external_temperature: f32) -> Self {
        ClimateControlSystem {
            current_temperature: initial_temperature,
            desired_temperature: initial_temperature,
            external_temperature,
        }
    }

    pub fn adjust_temperature(&mut self) {
        use std::cmp::Ordering;

        match self.current_temperature.partial_cmp(&self.desired_temperature).unwrap() {
            Ordering::Less => {
                self.current_temperature += 0.5;
                println!("Heating up. Current temperature: {:.1}°C", self.current_temperature);
            }
            Ordering::Greater => {
                self.current_temperature -= 0.5;
                println!("Cooling down. Current temperature: {:.1}°C", self.current_temperature);
            }
            Ordering::Equal => {
                println!("Desired temperature reached: {:.1}°C", self.current_temperature);
            }
        }
    }

    pub fn simulate_external_conditions(&mut self) {
        let mut rng = rand::thread_rng();

        // Randomly adjust external temperature
        self.external_temperature += rng.gen_range(-0.5..0.5);
        println!("External temperature changed to: {:.1}°C", self.external_temperature);

        // Randomly set a new desired temperature
        self.desired_temperature = rng.gen_range(18.0..26.0);
        println!("New desired temperature set to: {:.1}°C", self.desired_temperature);
    }
}

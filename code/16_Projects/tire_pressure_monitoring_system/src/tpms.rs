use rand::Rng;

#[derive(Debug)]
pub struct Tire {
    pressure: f32,
    is_safe: bool,
}

impl Tire {
    pub fn new(pressure: f32) -> Self {
        Self {
            pressure,
            is_safe: true,
        }
    }

    pub fn check_pressure(&mut self, safe_pressure: f32) {
        if self.pressure < safe_pressure {
            self.is_safe = false;
        } else {
            self.is_safe = true;
        }
    }

    pub fn status(&self) -> TireStatus {
        if self.is_safe {
            TireStatus::Safe
        } else {
            TireStatus::Unsafe
        }
    }

    pub fn adjust_pressure(&mut self, delta: f32) {
        self.pressure += delta;
    }
}

#[derive(Debug)]
pub enum TireStatus {
    Safe,
    Unsafe,
}

pub struct TPMS {
    tires: Vec<Tire>,
    safe_pressure: f32,
    dtc_triggered: bool,
}

impl TPMS {
    pub fn new(safe_pressure: f32, tire_pressures: Vec<f32>) -> Self {
        let tires = tire_pressures
            .into_iter()
            .map(Tire::new)
            .collect();

        Self {
            tires,
            safe_pressure,
            dtc_triggered: false,
        }
    }

    pub fn check_all_tires(&mut self) {
        self.dtc_triggered = false;  // Reset DTC flag before checking
        for tire in &mut self.tires {
            tire.check_pressure(self.safe_pressure);
            if !tire.is_safe {
                self.dtc_triggered = true;
            }
        }
    }

    pub fn is_dtc_triggered(&self) -> bool {
        self.dtc_triggered
    }

    pub fn display_warnings(&self) {
        for (i, tire) in self.tires.iter().enumerate() {
            match tire.status() {
                TireStatus::Safe => println!("Tire {}: Pressure is safe ({:.2} PSI)", i + 1, tire.pressure),
                TireStatus::Unsafe => println!("Tire {}: WARNING! Pressure is unsafe ({:.2} PSI)", i + 1, tire.pressure),
            }
        }
    }

    pub fn simulate_pressure_change(&mut self) {
        let mut rng = rand::thread_rng();

        for tire in &mut self.tires {
            let pressure_change: f32 = rng.gen_range(-0.5..0.5);
            tire.adjust_pressure(pressure_change);
        }
    }
}

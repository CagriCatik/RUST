pub struct Odometer {
    total_kilometers: f64,
    trip_meter: f64,
    fuel_consumed: f64,
    fuel_efficiency: f64, // in km per liter
}

impl Odometer {
    // Constructor for Odometer
    pub fn new(fuel_efficiency: f64) -> Odometer {
        Odometer {
            total_kilometers: 0.0,
            trip_meter: 0.0,
            fuel_consumed: 0.0,
            fuel_efficiency,
        }
    }

    // Method to simulate driving
    pub fn drive(&mut self, speed: f64, hours: f64) {
        let distance = speed * hours; // Distance = Speed * Time
        self.total_kilometers += distance;
        self.trip_meter += distance;
        self.fuel_consumed += distance / self.fuel_efficiency;
    }

    // Method to reset the trip meter
    pub fn reset_trip_meter(&mut self) {
        self.trip_meter = 0.0;
    }

    // Getter for total kilometers
    pub fn total_kilometers(&self) -> f64 {
        self.total_kilometers
    }

    // Getter for trip meter
    pub fn trip_meter(&self) -> f64 {
        self.trip_meter
    }

    // Getter for fuel consumed
    pub fn fuel_consumed(&self) -> f64 {
        self.fuel_consumed
    }

    // Method to display odometer readings
    pub fn display_kilometers(&self) {
        println!(
            "Total Kilometers: {:.2} km | Trip Meter: {:.2} km | Fuel Consumed: {:.2} liters",
            self.total_kilometers, self.trip_meter, self.fuel_consumed
        );
    }
}

mod tpms;

use std::thread;
use std::time::Duration;

fn main() {
    let safe_pressure = 30.0;
    let tire_pressures = vec![32.0, 28.5, 31.0, 29.0];

    let mut tpms = tpms::TPMS::new(safe_pressure, tire_pressures);

    // Simulation over time
    for _ in 0..10 {  // Run the simulation for 10 iterations
        tpms.check_all_tires();
        tpms.display_warnings();

        if tpms.is_dtc_triggered() {
            println!("DTC Triggered: One or more tires have unsafe pressure!");
        } else {
            println!("All tires are within the safe pressure range.");
        }

        tpms.simulate_pressure_change();

        // Wait for 1 second before the next iteration
        thread::sleep(Duration::from_secs(1));
    }

    println!("Simulation completed.");
}

use std::thread;
use std::time::Duration;

use crate::vehicle::Vehicle;
use crate::road_condition::RoadCondition;

pub fn run_simulation() {
    let mut vehicle = Vehicle::new();

    loop {
        let road_condition = RoadCondition::random();

        vehicle.update_speed();
        vehicle.update_road_slope();
        vehicle.update_tire_condition();

        let traction = vehicle.adjust_for_condition(match road_condition {
            RoadCondition::Dry => 1.0,
            RoadCondition::Wet => 0.7,
            RoadCondition::Icy => 0.3,
        });

        let stopping_distance = vehicle.calculate_stopping_distance(traction);

        println!("-----------------------------------");
        println!(
            "Road condition: {:?}, Speed: {:.1} km/h, Road Slope: {:.1} degrees, Tire Condition: {:.2}",
            road_condition, vehicle.speed, vehicle.road_slope, vehicle.tire_condition
        );
        println!(
            "Estimated stopping distance: {:.2} meters.",
            stopping_distance
        );
        println!("-----------------------------------");

        thread::sleep(Duration::from_secs(5));
    }
}

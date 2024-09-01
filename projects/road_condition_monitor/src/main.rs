mod vehicle;
mod road_condition;
mod simulation;

use simulation::run_simulation;

fn main() {
    println!("Starting Advanced Road Condition Simulator...");
    run_simulation();
}

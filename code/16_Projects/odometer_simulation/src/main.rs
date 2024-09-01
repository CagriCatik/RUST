mod odometer;
use odometer::Odometer;
use rand::Rng;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let mut odometer = Odometer::new(15.0);

    let total_hours = 24.0;
    let step = 0.5; // Every 30 minutes
    let mut hours_passed = 0.0;

    let mut time_data = vec![];
    let mut distance_data = vec![];
    let mut trip_data = vec![];
    let mut fuel_data = vec![];

    while hours_passed < total_hours {
        let speed: f64 = rng.gen_range(40.0..120.0); // Random speed between 40 and 120 km/h
        odometer.drive(speed, step);

        hours_passed += step;
        time_data.push(hours_passed);
        distance_data.push(odometer.total_kilometers());
        trip_data.push(odometer.trip_meter());
        fuel_data.push(odometer.fuel_consumed());
    }

    // Use the `display_kilometers` method to show the final readings
    odometer.display_kilometers();

    // Reset the trip meter at the end (this is just an example of using the method)
    odometer.reset_trip_meter();
    println!("Trip meter has been reset.");
    odometer.display_kilometers();

    plot_data(&time_data, &distance_data, &trip_data, &fuel_data)?;

    Ok(())
}


fn plot_data(
    time_data: &[f64],
    distance_data: &[f64],
    trip_data: &[f64],
    fuel_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("odometer_simulation.png", (1280, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((1, 3)); // Split into a 1x3 grid

    // Plot for Total Distance
    let mut chart1 = ChartBuilder::on(&areas[0])
        .caption("Total Distance (km) Over Time", ("sans-serif", 25))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..24.0, 0.0..distance_data.last().cloned().unwrap_or(0.0) + 10.0)?;

    chart1.configure_mesh().draw()?;

    chart1.draw_series(LineSeries::new(
        time_data.iter().zip(distance_data.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?
    .label("Total Distance (km)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &RED));

    // Plot for Trip Distance
    let mut chart2 = ChartBuilder::on(&areas[1])
        .caption("Trip Distance (km) Over Time", ("sans-serif", 25))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..24.0, 0.0..trip_data.last().cloned().unwrap_or(0.0) + 10.0)?;

    chart2.configure_mesh().draw()?;

    chart2.draw_series(LineSeries::new(
        time_data.iter().zip(trip_data.iter()).map(|(&x, &y)| (x, y)),
        &BLUE,
    ))?
    .label("Trip Distance (km)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &BLUE));

    // Plot for Fuel Consumed
    let mut chart3 = ChartBuilder::on(&areas[2])
        .caption("Fuel Consumed (liters) Over Time", ("sans-serif", 25))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..24.0, 0.0..fuel_data.last().cloned().unwrap_or(0.0) + 1.0)?;

    chart3.configure_mesh().draw()?;

    chart3.draw_series(LineSeries::new(
        time_data.iter().zip(fuel_data.iter()).map(|(&x, &y)| (x, y)),
        &GREEN,
    ))?
    .label("Fuel Consumed (liters)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], &GREEN));

    // Display the series labels (legends)
    chart1.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;
    chart2.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;
    chart3.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;

    Ok(())
}

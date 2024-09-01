use rand::Rng;

pub struct Vehicle {
    pub speed: f32,
    pub braking_efficiency: f32,
    pub tire_condition: f32,
    pub road_slope: f32,
}

impl Vehicle {
    pub fn new() -> Self {
        Vehicle {
            speed: 50.0,
            braking_efficiency: 0.9,
            tire_condition: 0.9,
            road_slope: 0.0,
        }
    }

    pub fn adjust_for_condition(&self, traction: f32) -> f32 {
        let mut adjusted_traction = traction * self.tire_condition;

        if self.road_slope > 0.0 {
            adjusted_traction *= 1.0 - (self.road_slope / 45.0);
        } else if self.road_slope < 0.0 {
            adjusted_traction *= 1.0 + (-self.road_slope / 45.0);
        }

        adjusted_traction
    }

    pub fn calculate_stopping_distance(&self, traction: f32) -> f32 {
        let velocity = self.speed / 3.6;
        let gravity = 9.81;
        (velocity * velocity) / (2.0 * traction * gravity * self.braking_efficiency)
    }

    pub fn update_speed(&mut self) {
        let mut rng = rand::thread_rng();
        let speed_change: f32 = rng.gen_range(-10.0..10.0);
        self.speed = (self.speed + speed_change).clamp(0.0, 150.0);
    }

    pub fn update_road_slope(&mut self) {
        let mut rng = rand::thread_rng();
        let slope_change: f32 = rng.gen_range(-5.0..5.0);
        self.road_slope = (self.road_slope + slope_change).clamp(-10.0, 10.0);
    }

    pub fn update_tire_condition(&mut self) {
        let mut rng = rand::thread_rng();
        let wear: f32 = rng.gen_range(-0.02..0.0);
        self.tire_condition = (self.tire_condition + wear).clamp(0.5, 1.0);
    }
}

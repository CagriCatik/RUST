#[derive(Debug)]
pub enum RoadCondition {
    Dry,
    Wet,
    Icy,
}

impl RoadCondition {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => RoadCondition::Dry,
            1 => RoadCondition::Wet,
            _ => RoadCondition::Icy,
        }
    }
}

use ABC_ECS::Resource;

pub struct DeltaTime {
    start: std::time::Instant,
    last_frame_time: std::time::Duration,
    delta_time: f64,
    time_scale: f64,
}

impl DeltaTime {
    pub fn new() -> Self {
        let start = std::time::Instant::now();
        let last_frame_time = start.elapsed();
        let delta_time = 0.0;
        Self {
            start,
            last_frame_time,
            delta_time,
            time_scale: 1.0,
        }
    }

    pub fn set_time_scale(&mut self, time_scale: f64) {
        self.time_scale = time_scale;
    }

    pub fn get_time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn get_delta_time(&self) -> f64 {
        self.delta_time * self.time_scale
    }
}

impl Resource for DeltaTime {
    fn update(&mut self) {
        let current_frame_time = self.start.elapsed();
        self.delta_time = (current_frame_time - self.last_frame_time).as_secs_f64();

        // Do something with delta_time...

        self.last_frame_time = current_frame_time;
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

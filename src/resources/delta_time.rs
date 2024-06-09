use ABC_ECS::Resource;

pub struct DeltaTime {
    start: std::time::Instant,
    last_frame_time: std::time::Duration,
    delta_time: f64,
    correctional_delta_time: f64, // This is used to correct the delta time for when the time scale is changed mid-frame
    time_scale: f64,
    total_time: f64,
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
            correctional_delta_time: 0.0,
            total_time: 0.0,
        }
    }

    pub fn set_time_scale(&mut self, time_scale: f64) {
        // probably won't happen but just in case the time scale is changed multiple times in a frame
        self.correctional_delta_time += self.delta_time * self.time_scale;
        self.delta_time = 0.0;

        self.time_scale = time_scale;
    }

    pub fn get_time_scale(&self) -> f64 {
        self.time_scale
    }

    pub fn get_delta_time(&self) -> f64 {
        (self.delta_time * self.time_scale) + self.correctional_delta_time
    }

    pub fn get_total_time(&self) -> f64 {
        self.total_time
    }
}

impl Resource for DeltaTime {
    fn update(&mut self) {
        let current_frame_time = self.start.elapsed();
        self.delta_time = (current_frame_time - self.last_frame_time).as_secs_f64();
        self.correctional_delta_time = 0.0;

        self.total_time += self.delta_time * self.time_scale;

        self.last_frame_time = current_frame_time;
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

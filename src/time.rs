use std::time::{Duration, Instant};

pub struct Time {
    last_frame: Instant,
    delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
            delta: Duration::ZERO,
        }
    }
}

impl Time {
    pub fn update(&mut self) {
        let time = Instant::now();
        self.delta = time.duration_since(self.last_frame);
        self.last_frame = time;
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}

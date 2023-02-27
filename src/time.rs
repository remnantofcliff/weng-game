#![allow(unused)]

pub struct Time {
    accumulator: std::time::Duration,
    last_time: std::time::Instant,
}

impl Time {
    pub const TICK_RATE: std::time::Duration = std::time::Duration::from_nanos(16_666_666);

    pub fn begin_loop(&mut self) {
        let time = std::time::Instant::now();
        self.accumulator += time - self.last_time;
        self.last_time = time;
    }

    pub fn blend_factor(&self) -> f64 {
        self.accumulator.as_secs_f64() / Self::TICK_RATE.as_secs_f64()
    }

    pub fn new() -> Self {
        Self {
            accumulator: std::time::Duration::ZERO,
            last_time: std::time::Instant::now(),
        }
    }

    pub fn should_update(&self) -> bool {
        self.accumulator >= Self::TICK_RATE
    }

    pub fn update(&mut self) {
        self.accumulator -= Self::TICK_RATE;
    }
}

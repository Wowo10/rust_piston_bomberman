use std::time::Instant;

pub struct Timer {
    time: Instant,
}

impl Timer {
    pub fn create() -> Self {
        Timer {
            time: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.time = Instant::now();
    }

    pub fn get_elapsed(&self) -> u64 {
        let duration = Instant::now().duration_since(self.time);

        (duration.as_secs() * 1000) + duration.subsec_millis() as u64
    }

    pub fn did_pass(&self, duration: u64) -> bool {
        self.get_elapsed() > duration
    }

    pub fn progress(&self, duration: u64) -> f64 {
        let percentage = self.get_elapsed() as f64 / duration as f64;

        if percentage > 1.0 {
            1.0
        } else {
            percentage
        }
    }
}

pub struct Timers {
    pub updatetimer: Timer,
}

pub fn new_timers() -> Timers {
    Timers {
        updatetimer: Timer::create(),
    }
}

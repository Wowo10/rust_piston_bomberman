use app::timers::*;

pub struct Fire {
    pub position: [u8; 2],
    pub fire_timer: Timer,
    pub time_limit: f32,
}

impl Fire {
    pub fn create(position: [u8; 2], time_limit: f32) -> Self {
        Fire {
            fire_timer: Timer::create(),
            position: position,
            time_limit: time_limit,
        }
    }

    pub fn ended(&self) -> bool {
        self.fire_timer.did_pass((self.time_limit * 1000.0) as u64)
    }

    pub fn get_position(&self) -> [u8; 2] {
        self.position
    }
}

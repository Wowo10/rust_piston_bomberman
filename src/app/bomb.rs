use app::timers::*;

pub struct Bomb {
    pub fire_range: u8,
    pub boom_timer: Timer,
    pub position: [u8; 2],
    pub time_limit: u32,
}

impl Bomb {
    pub fn create(position: [u8; 2], time_limit: u32) -> Self {
        Bomb {
            fire_range: 1,
            boom_timer: Timer::create(),
            position: position,
            time_limit: time_limit,
        }
    }

    pub fn get_percentage(&self) -> f64 {
        self.boom_timer.progress((self.time_limit * 1000) as u64)
    }

    pub fn get_position(&self) -> [u8; 2] {
        self.position
    }
}

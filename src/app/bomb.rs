use app::timers::*;

pub struct Bomb {
    pub fire_range: u8,
    pub boom_timer: Timer,
    pub position: [u8; 2],
}

impl Bomb {
    pub fn create(position: [u8; 2]) -> Self {
        Bomb {
            fire_range: 1,
            boom_timer: Timer::create(),
            position: position,
        }
    }

    pub fn get_position(&self) -> [u8; 2] {
        self.position
    }
}

use app::timers;

pub struct Bomb {
    pub fire_range: u8,
    pub boom_timer: timers::Timer,
    pub position: [u8; 2],
}

impl Bomb {
    pub fn get_position(&self) -> [u8; 2] {
        self.position
    }
}

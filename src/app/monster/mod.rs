use app::timers::*;

pub struct Monster {
    pub position: [u8; 2],
    pub move_timer: Timer,
    pub time_limit: f32,
}

impl Monster {
    pub fn create(position: [u8; 2]) -> Self {
        Monster {
            position: position,
            move_timer: Timer::create(),
            time_limit: 0.5,
        }
    }
}

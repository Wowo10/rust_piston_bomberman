pub struct Statistics {
    pub bomb_limit: u8,
    pub active_bombs: u8,

    pub time_limit: u8,
    pub fire_range: u8,
}

impl Statistics {
    pub fn create() -> Self {
        Statistics {
            bomb_limit: 1,
            active_bombs: 0,

            time_limit: 5,
            fire_range: 1,
        }
    }
}
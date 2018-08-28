pub enum State {
    Free,
    Fire,
    Obstacle,
    Block,
}

impl Copy for State {}

impl Clone for State {
    fn clone(&self) -> State {
        *self
    }
}

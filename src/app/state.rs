use std::fmt;

pub enum State {
    Free,
    Obstacle,
    Block,
}

impl Copy for State {}

impl Clone for State {
    fn clone(&self) -> State {
        *self
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Free => "Free",
                State::Block => "Block",
                State::Obstacle => "Obstacle",
            }
        )
    }
}

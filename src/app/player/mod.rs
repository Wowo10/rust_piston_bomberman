mod control;
pub mod statistics;
use self::statistics::*;
use piston_window::Key;

pub struct Player {
    pub position: [u8; 2],
    pub controls: control::Control,
    pub color: [f32; 4],

    pub statistics: Statistics,
}

impl Player {
    pub fn move_left(&mut self) {
        self.position[0] -= 1;
    }
    pub fn move_up(&mut self) {
        self.position[1] -= 1;
    }
    pub fn move_right(&mut self) {
        self.position[0] += 1;
    }
    pub fn move_down(&mut self) {
        self.position[1] += 1;
    }

    pub fn lay_bomb(&mut self) -> bool {
        let condition = self.statistics.active_bombs < self.statistics.bomb_limit;

        if condition {
            self.statistics.active_bombs += 1;
        };

        condition
    }

    pub fn bomb_exploded(&mut self) {
        self.statistics.active_bombs -= 1;
    }

    pub fn get_position(&self) -> [u8; 2] {
        self.position
    }
}

pub fn create_controls(index: u8) -> control::Control {
    //hardcoded for 2 players
    control::Control {
        up_button: if index == 0 { Key::Up } else { Key::W },
        down_button: if index == 0 { Key::Down } else { Key::S },
        left_button: if index == 0 { Key::Left } else { Key::A },
        right_button: if index == 0 { Key::Right } else { Key::D },
        bomb_button: if index == 0 { Key::RCtrl } else { Key::LCtrl },
    }
}

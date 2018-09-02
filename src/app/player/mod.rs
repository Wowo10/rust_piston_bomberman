mod control;
use piston_window::*;

pub struct Player {
    pub position: [u8; 2],
    pub controls: control::Control,
    pub color: [f32; 4],
}

impl Player {
    fn able_move_left(&self) -> bool {
        true
    }

    pub fn get_position(&self) -> [u8; 2]{
        self.position
    }

    // fn move_left(&mut self) {
    //     let possible = self.able_move_left();
    //     if possible {
    //         for block in &mut self.activeblock {
    //             block[0] -= 1;
    //         }
    //     }
    // }

    // fn able_move_right(&self) -> bool {
    //     for block in &self.activeblock {
    //         let x_pos = block[0];
    //         if x_pos == (&self.scene.len() - 1) as u8 {
    //             return false;
    //         }
    //         match &self.scene[(x_pos + 1) as usize][block[1] as usize] {
    //             State::Taken => {
    //                 return false;
    //             }
    //             _ => {}
    //         }
    //     }
    //     true
    // }

    // fn move_right(&mut self) {
    //     let possible = self.able_move_right();
    //     if possible {
    //         for block in &mut self.activeblock {
    //             block[0] += 1;
    //         }
    //     }
    // }

    // fn able_move_down(&self) -> bool {
    //     for block in &self.activeblock {
    //         let y_pos = block[1];
    //         if y_pos == (&self.scene[0].len() - 1) as u8 {
    //             return false;
    //         }
    //         match &self.scene[block[0] as usize][(block[1] + 1) as usize] {
    //             State::Taken => {
    //                 return false;
    //             }
    //             _ => {}
    //         }
    //     }
    //     true
    // }

    // fn move_down(&mut self) {
    //     let possible = self.able_move_down();

    //     if possible {
    //         for block in &mut self.activeblock {
    //             block[1] += 1;
    //         }
    //     }
    //     self.timers.updatetimer.reset();
    // }
}

pub fn CreateControls(index: u8) -> control::Control{ //hardcoded for 2 players
    control::Control{
        up_button: if index == 0 {Key::Up} else {Key::W},
        down_button: if index == 0 {Key::Down} else {Key::S},
        left_button: if index == 0 {Key::Left} else {Key::A},
        right_button: if index == 0 {Key::Right} else {Key::D},
        bomb_button: if index == 0 {Key::RCtrl} else {Key::LCtrl}
    }
}
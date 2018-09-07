extern crate find_folder;
extern crate piston_window;

use piston_window::*;

mod state;
use self::state::State;

extern crate rand;
use app::rand::prelude::*;

pub mod config;
pub mod timers;
use self::timers::Timers;

mod map;

mod constants;
mod vector_operations;
use self::constants::Constants;

mod player;

pub struct App {
    pub scene: Vec<Vec<State>>,

    pub renderframes: usize,
    pub updateframes: usize,
    pub timers: Timers,

    pub exit: bool,

    pub players: Vec<player::Player>,
    pub settings: Constants,
}

impl App {
    pub fn create(
        max_width: u32,
        max_height: u32,
        color_background: [f32; 4],
        color_border: [f32; 4],
        color_block: [f32; 4],
        color_obstacle: [f32; 4],
        color_player1: [f32; 4],
        color_player2: [f32; 4],
        offset: u8,
        players_amount: u8,
    ) -> Self {
        let mut temp = App {
            scene: Vec::new(),
            renderframes: 0,
            updateframes: 0,
            timers: timers::new_timers(),
            exit: false,
            players: Vec::new(),

            settings: Constants {
                color_background: color_background,
                color_border: color_border,
                color_block: color_block,
                color_obstacle: color_obstacle,
                color_player1: color_player1,
                color_player2: color_player2,
                size_x: 1,
                size_y: 1,
                offset: offset,
            },
        };

        let [size_x, size_y] = temp.init(players_amount);
        temp.settings.size_x = max_width / size_x as u32;
        temp.settings.size_y = max_height / size_y as u32;
        temp
    }

    fn init(&mut self, players_amount: u8) -> [usize; 2] {
        self.read_map(players_amount)

        //self.gen_players(players_amount);
    }

    /// 0 for free
    /// 1 for random
    /// 2 for block
    /// 3 for obstacle
    /// 4 for playerpos (to be implemented)
    ///
    fn read_map(&mut self, players_amount: u8) -> [usize; 2] {
        let temp = map::read_map("map.csv");

        for i in 0..temp[0].len() {
            let mut v: Vec<State> = Vec::new();
            for j in 0..temp.len() {
                v.push(match temp[j][i] {
                    '1' => {
                        let mut rng = thread_rng();

                        let rand_state: u8 = rng.gen();

                        match rand_state % 3 {
                            0 => State::Free,
                            1 => State::Block,
                            2 => State::Obstacle,
                            _ => State::Free,
                        }
                    }
                    '2' => State::Block,
                    '3' => State::Obstacle,
                    '4' => {
                        if players_amount >= self.players.len() as u8 {
                            &self.add_player([i as u8, j as u8]);
                        }
                        State::Free
                    }
                    _ => State::Free, //'0'
                });
            }
            &self.scene.push(v);
        }

        [self.scene.len(), self.scene[0].len()]
    }

    fn gen_players(&mut self, players_amount: u8) {
        for i in 0..players_amount {
            //read us from config!

            let len = self.scene.len();

            self.add_player(if i == 0 { [0, 0] } else { [(len - 1) as u8, 0] });
        }
    }

    fn add_player(&mut self, player_position: [u8; 2]) {
        let i = self.players.len();

        self.players.push(player::Player {
            position: player_position,
            color: if i == 0 {
                self.settings.color_player1
            } else {
                self.settings.color_player2
            },
            controls: player::create_controls(i as u8),
            statistics: player::Statistics::create(),
        });
    }

    pub fn render(&mut self, window: &mut PistonWindow, e: Input, _args: RenderArgs) {
        self.renderframes += 1;

        let square = [
            (self.settings.offset / 2) as f64,
            (self.settings.offset / 2) as f64,
            (self.settings.size_x - self.settings.offset as u32) as f64,
            (self.settings.size_y - self.settings.offset as u32) as f64,
        ];

        let size_x = self.settings.size_x as f64;
        let size_y = self.settings.size_y as f64;

        let width = self.scene.len();
        let heigth = self.scene[0].len();

        let scene = &self.scene;

        let players = &self.players;

        window.draw_2d(&e, |c, g| {
            clear(self.settings.color_border, g);

            for i in 0..width {
                for j in 0..heigth {
                    let transposition = c.transform.trans(size_x * i as f64, size_y * j as f64);

                    let mut color = match &scene[i][j] {
                        State::Block => self.settings.color_block,
                        State::Obstacle => self.settings.color_obstacle,

                        _ => self.settings.color_background,
                    };

                    for player in players {
                        if [i as u8, j as u8] == player.get_position() {
                            color = player.color;
                        }
                    }

                    rectangle(color, square, transposition, g);
                }
            }
        });
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.updateframes += 1;

        self.exit = self.players.is_empty();
    }

    pub fn handle_input(&mut self, key: Key) {
        for i in 0..self.players.len() {
            if key == self.players[i].controls.up_button {
                if self.able_move_up(i) {
                    self.players[i].move_up();
                }
            } else if key == self.players[i].controls.down_button {
                if self.able_move_down(i) {
                    self.players[i].move_down();
                }
            } else if key == self.players[i].controls.left_button {
                if self.able_move_left(i) {
                    self.players[i].move_left();
                }
            } else if key == self.players[i].controls.right_button {
                if self.able_move_right(i) {
                    self.players[i].move_right();
                }
            } else if key == self.players[i].controls.bomb_button {
                if true {
                    //also check for timers etc
                    self.players[i].lay_bomb();
                }
            }
        }
    }

    //Move checkings
    fn check_scene_state(&self, pos_x: usize, pos_y: usize) -> bool {
        match &self.scene[pos_x][pos_y] {
            State::Block | State::Obstacle => true,
            _ => false,
        }
    }

    fn check_other_players(&self, pos_x: u8, pos_y: u8) -> bool {
        for player in &self.players {
            let [other_player_x, other_player_y] = player.get_position();

            if pos_x == other_player_x && other_player_y == pos_y {
                return true;
            }
        }
        false
    }

    fn check_scene_and_players(&self, pos_x: u8, pos_y: u8) -> bool {
        self.check_scene_state(pos_x as usize, pos_y as usize)
            || self.check_other_players(pos_x, pos_y)
    }

    fn able_move_left(&self, player_index: usize) -> bool {
        let [player_pos_x, player_pos_y] = self.players[player_index].get_position();

        if player_pos_x <= 0 {
            return false;
        }

        if self.check_scene_and_players(player_pos_x - 1, player_pos_y) {
            return false;
        }

        true
    }
    fn able_move_right(&self, player_index: usize) -> bool {
        let [player_pos_x, player_pos_y] = self.players[player_index].get_position();

        if player_pos_x as usize >= (self.scene.len() - 1) {
            return false;
        }

        if self.check_scene_and_players(player_pos_x + 1, player_pos_y) {
            return false;
        }

        true
    }
    fn able_move_up(&self, player_index: usize) -> bool {
        let [player_pos_x, player_pos_y] = self.players[player_index].get_position();

        if player_pos_y <= 0 {
            return false;
        }

        if self.check_scene_and_players(player_pos_x, player_pos_y - 1) {
            return false;
        }

        true
    }
    fn able_move_down(&self, player_index: usize) -> bool {
        let [player_pos_x, player_pos_y] = self.players[player_index].get_position();

        if player_pos_y as usize >= self.scene[0].len() - 1 {
            return false;
        }

        if self.check_scene_and_players(player_pos_x, player_pos_y + 1) {
            return false;
        }

        true
    }
}

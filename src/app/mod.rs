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
use self::vector_operations::*;

mod bomb;
use self::bomb::*;
mod fire;
use self::fire::*;
mod player;
use self::player::statistics::*;
mod monster;
use self::monster::*;
pub mod powerup;
use self::powerup::*;

pub struct App {
    pub scene: Vec<Vec<State>>,

    pub renderframes: usize,
    pub updateframes: usize,
    pub timers: Timers,

    pub exit: bool,

    pub players: Vec<player::Player>,
    pub monsters: Vec<Monster>,
    pub bombs: Vec<Bomb>,
    pub fires: Vec<Fire>,
    pub powerups: Vec<Powerup>,
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
        color_bomb: [f32; 4],
        color_fire: [f32; 4],
        color_powerup_bomb: [f32; 4],
        color_powerup_fire: [f32; 4],
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
            monsters: Vec::new(),
            bombs: Vec::new(),
            fires: Vec::new(),
            powerups: Vec::new(),

            settings: Constants {
                color_background: color_background,
                color_border: color_border,
                color_block: color_block,
                color_obstacle: color_obstacle,
                color_player1: color_player1,
                color_player2: color_player2,
                color_bomb: color_bomb,
                color_fire: color_fire,
                color_powerup_bomb: color_powerup_bomb,
                color_powerup_fire: color_powerup_fire,
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
                    '5' => State::Free,
                    _ => State::Free, //'0'
                });
            }
            &self.scene.push(v);
        }

        [self.scene.len(), self.scene[0].len()]
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
            dead: false,
            controls: player::create_controls(i as u8),
            statistics: Statistics::create(),
        });
    }

    fn add_monster(&mut self, monster_position: [u8; 2]) {
        self.monsters.push(Monster::create(monster_position));
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
        let bombs = &self.bombs;
        let fires = &self.fires;
        let powerups = &self.powerups;

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

                    for bomb in bombs {
                        if [i as u8, j as u8] == bomb.get_position() {
                            color = color_lerp(
                                self.settings.color_bomb,
                                self.settings.color_fire,
                                bomb.get_percentage(),
                            );
                        }
                    }

                    for powerup in powerups {
                        if [i as u8, j as u8] == powerup.get_position() {
                            color = match powerup.get_type() {
                                powerup::Type::BonusBomb => self.settings.color_powerup_bomb,
                                powerup::Type::BonusFire => self.settings.color_powerup_fire,
                            };
                        }
                    }

                    for fire in fires {
                        if [i as u8, j as u8] == fire.get_position() {
                            color = self.settings.color_fire;
                        }
                    }

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

        self.clear_bombs();

        self.check_collisions();

        self.clear_queues();
    }

    fn clear_bombs(&mut self) {
        let players = &mut self.players;
        let fire = &mut self.fires;
        let scene = &mut self.scene;
        let powerups = &mut self.powerups;

        self.bombs.retain(|ref bomb| {
            if bomb.exploded() {
                players[bomb.player_number as usize].bomb_exploded();

                let [x_pos, y_pos] = bomb.get_position();
                let range = players[bomb.player_number as usize].statistics.fire_range as i8;
                let time = 0.5; //get from config?

                fire.push(Fire::create([x_pos, y_pos], time));

                App::fire_spread_left(scene, fire, powerups, range, x_pos as i8, y_pos, time);
                App::fire_spread_right(scene, fire, powerups, range, x_pos as i8, y_pos, time);
                App::fire_spread_up(scene, fire, powerups, range, x_pos, y_pos as i8, time);
                App::fire_spread_down(scene, fire, powerups, range, x_pos, y_pos as i8, time);

                false
            } else {
                true
            }
        });
    }

    fn fire_spread_left(
        scene: &mut Vec<Vec<State>>,
        fire: &mut Vec<Fire>,
        powerups: &mut Vec<Powerup>,
        range: i8,
        x_pos: i8,
        y_pos: u8,
        time: f32,
    ) {
        for i in 1..(range + 1) {
            let updated_x_pos = x_pos - i;
            if updated_x_pos >= 0 {
                if App::match_tile(scene, fire, powerups, updated_x_pos as u8, y_pos, time) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn fire_spread_right(
        scene: &mut Vec<Vec<State>>,
        fire: &mut Vec<Fire>,
        powerups: &mut Vec<Powerup>,
        range: i8,
        x_pos: i8,
        y_pos: u8,
        time: f32,
    ) {
        for i in 1..(range + 1) {
            let updated_x_pos = x_pos as i8 + i;
            if updated_x_pos < scene.len() as i8 {
                if App::match_tile(scene, fire, powerups, updated_x_pos as u8, y_pos, time) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn fire_spread_up(
        scene: &mut Vec<Vec<State>>,
        fire: &mut Vec<Fire>,
        powerups: &mut Vec<Powerup>,
        range: i8,
        x_pos: u8,
        y_pos: i8,
        time: f32,
    ) {
        for i in 1..(range + 1) {
            let updated_y_pos = y_pos as i8 - i;
            if updated_y_pos >= 0 {
                if App::match_tile(scene, fire, powerups, x_pos, updated_y_pos as u8, time) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn fire_spread_down(
        scene: &mut Vec<Vec<State>>,
        fire: &mut Vec<Fire>,
        powerups: &mut Vec<Powerup>,
        range: i8,
        x_pos: u8,
        y_pos: i8,
        time: f32,
    ) {
        for i in 1..(range + 1) {
            let updated_y_pos = y_pos as i8 + i;
            if updated_y_pos < scene[0].len() as i8 {
                if App::match_tile(scene, fire, powerups, x_pos, updated_y_pos as u8, time) {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn match_tile(
        scene: &mut Vec<Vec<State>>,
        fire: &mut Vec<Fire>,
        powerups: &mut Vec<Powerup>,
        x_pos: u8,
        y_pos: u8,
        time: f32,
    ) -> bool {
        let mut rng = thread_rng();

        let rand_num: u8 = rng.gen();

        match scene[x_pos as usize][y_pos as usize] {
            State::Free => {
                fire.push(Fire::create([x_pos, y_pos], time));
                false
            }
            State::Block => {
                scene[x_pos as usize][y_pos as usize] = State::Free;
                fire.push(Fire::create([x_pos, y_pos], time));

                if rand_num % 5 == 0 {
                    let choose: bool = rng.gen();
                    powerups.push(Powerup::create(
                        [x_pos, y_pos],
                        if choose {
                            powerup::Type::BonusBomb
                        } else {
                            powerup::Type::BonusFire
                        },
                    ));
                }
                true
            }
            State::Obstacle => true,
        }
    }

    fn check_collisions(&mut self) {
        self.exit = true; //assume that we end the game

        for player in &mut self.players {
            if !player.dead {
                if self.exit {
                    //and if there is any player not dead change to false
                    self.exit = false;
                }

                for fire in &self.fires {
                    if player.get_position() == fire.get_position() {
                        player.die();
                    }
                }

                for powerup in &mut self.powerups {
                    if player.get_position() == powerup.get_position() {
                        player.collect(powerup.get_type());
                        powerup.collect();
                    }
                }
            }
        }
    }

    fn clear_queues(&mut self) {
        self.fires.retain(|ref fire| !fire.ended());
        self.powerups.retain(|ref powerup| !powerup.collected);
    }

    pub fn handle_input(&mut self, key: Key) {
        for i in 0..self.players.len() {
            if !self.players[i].dead {
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
                    if self.players[i].lay_bomb() {
                        if !self.is_bomb_in_position(self.players[i].get_position()) {
                            self.bombs.push(Bomb::create(
                                self.players[i].get_position(),
                                i as u8,
                                2.0,
                            ));
                        }
                    };
                }
            }
        }
    }

    fn is_bomb_in_position(&self, position: [u8; 2]) -> bool {
        for bomb in &self.bombs {
            if bomb.get_position() == position {
                return true;
            }
        }

        false
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

    fn check_bombs(&self, pos_x: u8, pos_y: u8) -> bool {
        for bomb in &self.bombs {
            let [bomb_x, bomb_y] = bomb.get_position();

            if pos_x == bomb_x && bomb_y == pos_y {
                return true;
            }
        }
        false
    }

    fn check_scene_and_players(&self, pos_x: u8, pos_y: u8) -> bool {
        self.check_scene_state(pos_x as usize, pos_y as usize)
            || self.check_other_players(pos_x, pos_y)
            || self.check_bombs(pos_x, pos_y)
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

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

mod constants;
mod vector_operations;
use self::constants::Constants;

mod player;

pub struct App {
    pub scene: Vec<Vec<State>>,

    pub renderframes: usize,
    pub updateframes: usize,
    pub timers: Timers,

    pub activeblock: Vec<[u8; 2]>,

    pub exit: bool,
    pub score: u32,

    pub players: Vec<player::Player>,
    pub settings: Constants,
}

impl App {
    pub fn create(
        size: u32,
        width: u8,
        height: u8,
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
            activeblock: Vec::new(),
            exit: false,
            score: 0,
            players: Vec::new(),

            settings: Constants {
                color_background: color_background,
                color_border: color_border,
                color_block: color_block,
                color_obstacle: color_obstacle,
                color_player1: color_player1,
                color_player2: color_player2,
                size: size,
                offset: offset,
            },
        };

        temp.init(width, height, players_amount);
        temp
    }

    fn init(&mut self, width: u8, height: u8, players_amount: u8) {
        for _ in 0..width {
            let mut v: Vec<State> = Vec::new();
            for _ in 0..height {
                v.push(State::Free);
            }
            &self.scene.push(v);
        }

        for i in 0..players_amount {
            //read us from config!

            let temp: player::Player;

            temp = player::Player{
                position: if i == 0 {[0,0]} else {[self.scene.len() as u8, 0]},
                controls: player::CreateControls(i),
            };

            self.players.push(temp);
        }
    }

    fn clear_board(&mut self) {
        for row in &mut self.scene {
            for state in row {
                match *state {
                    _ => {}
                }
            }
        }
    }

    pub fn render(&mut self, window: &mut PistonWindow, e: Input, _args: RenderArgs) {
        self.renderframes += 1;

        let square = rectangle::square(0.0, 0.0, self.settings.size as f64);
        let squareinner = rectangle::square(
            (self.settings.offset / 2) as f64,
            (self.settings.offset / 2) as f64,
            (self.settings.size - self.settings.offset as u32) as f64,
        );

        let size = (self.settings.size as f64) / 2.0;

        let width = self.scene.len();
        let heigth = self.scene[0].len();

        let scene = &self.scene;

        window.draw_2d(&e, |c, g| {
            clear(self.settings.color_border, g);

            for i in 0..width {
                for j in 0..heigth {
                    let transposition = c.transform
                        .trans(size * 2.0 * i as f64, size * 2.0 * j as f64);

                    rectangle(self.settings.color_border, square, transposition, g);

                    let color = match &scene[i][j] {
                        _ => self.settings.color_background,
                    };

                    rectangle(color, squareinner, transposition, g);
                }
            }
        });
    }

    pub fn update(&mut self, _args: UpdateArgs) {
        self.updateframes += 1;

        self.clear_board();

        // for block in &mut self.activeblock {
        //     self.scene[block[0] as usize][block[1] as usize] = State::Active;
        // }

        self.exit = self.players.is_empty();
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Left | Key::A => {}
            Key::Right | Key::D => {}
            Key::Down | Key::S => {}
            Key::Up | Key::W => {}
            Key::Space => {}

            _ => {}
        }
    }
}

extern crate find_folder;
extern crate piston_window;
extern crate rand;

use piston_window::*;

mod app;
use app::config;

mod configuration;

use app::timers::*;

fn main() {
    let start = Timer::create();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut config = config::Config::create("config.csv");

    let width: u32 = config.read("width").parse::<u32>().unwrap();
    let height: u32 = config.read("height").parse::<u32>().unwrap();

    let mut window: PistonWindow = WindowSettings::new("WowoBomberman", [width, height])
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = app::App::create(
        width,
        height,
        config.read_color("background_color"),
        config.read_color("border_color"),
        config.read_color("block_color"),
        config.read_color("obstacle_color"),
        config.read_color("player1_color"),
        config.read_color("player2_color"),
        config.read_color("bomb_color"),
        config.read_color("fire_color"),
        config.read_color("powerup_bomb_color"),
        config.read_color("powerup_fire_color"),
        config.read("offset").parse::<u8>().unwrap(),
        config.read("players").parse::<u8>().unwrap(),
    );

    while let Some(e) = window.next() {
        match e {
            Input::Release(Button::Keyboard(key)) => {
                app.handle_input(key);
            }

            Input::Update(args) => {
                app.update(args);
            }

            Input::Render(args) => {
                app.render(&mut window, e, args);
            }

            _ => {}
        }

        if app.exit {
            break;
        };
    }

    let duration = start.get_elapsed() as f64 / 1000.0;

    println!(
        "update: {}, render: {}, update/s:{}, render/s:{}, duration:{}s",
        app.updateframes,
        app.renderframes,
        (app.updateframes as f64 / duration) as usize,
        (app.renderframes as f64 / duration) as usize,
        duration
    );

    println!("\nGame Over!");

    configuration::set(1);
    println!("config: {}", configuration::read());
}

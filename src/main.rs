#![windows_subsystem = "windows"]

mod ball;
mod paddle;
mod score;
mod game;
mod logic;

use game::*;
use logic::game_logic;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        game_logic(&mut game);
        next_frame().await
    }
}
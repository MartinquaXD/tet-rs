use crate::game::Game;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

mod game;
mod views;

#[tokio::main]
async fn main() {
    enable_raw_mode().unwrap();
    let game = Arc::new(Mutex::new(Game::default()));
    let read_input = Game::read_input(game.clone());
    let render = Game::render(game.clone());
    spawn(render);
    read_input.await;
    disable_raw_mode().unwrap();
    println!("");
}

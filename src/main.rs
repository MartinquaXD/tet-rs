#![feature(clamp, async_closure)]

use crate::game::Game;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::views::play_view::view::PlayView;

mod game;
mod views;
mod renderer;

#[tokio::main]
async fn main() {
    enable_raw_mode().unwrap();
    let game = Arc::new(Mutex::new(Game::default()));
    let read_input = Game::read_input(game.clone());
    let render = Game::render(game.clone());
    spawn(PlayView::generate_ticks(game.clone()));
    spawn(render);
    read_input.await;
    disable_raw_mode().unwrap();
    println!("");
}


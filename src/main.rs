#![feature(clamp, async_closure, drain_filter)]
#![allow(dead_code)]

use crate::game::Game;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

mod game;
mod views;
mod rendering;

#[tokio::main]
async fn main() {
    enable_raw_mode().unwrap();
    let game = Game::create();
    Game::run(game).await;
    disable_raw_mode().unwrap();
    println!("{}", termion::cursor::Show);
}


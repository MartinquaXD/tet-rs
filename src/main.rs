#![feature(clamp, async_closure, drain_filter)]
#![allow(dead_code)]


use crate::game::Game;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use crossterm::cursor::{Show, Hide};
use crossterm::execute;
use crossterm::Result;
use std::io::{stdout, Write};

mod game;
mod views;
mod rendering;

#[tokio::main]
async fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;
    let game = Game::create();
    Game::run(game).await;
    disable_raw_mode()?;
    execute!(stdout(), Clear(ClearType::All), LeaveAlternateScreen, Show)?;
    Ok(())
}


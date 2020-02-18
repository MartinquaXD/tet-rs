use super::field;
use termion::{color, cursor};
use termion::color::Bg;
use tokio::io::{AsyncWriteExt, AsyncWrite};
use crate::renderer::{Texture, Position, Canvas, Dimensions, Color};
use super::stones::Stone;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use crate::game::Game;
use tokio::time::delay_for;

pub struct PlayView {
    field: field::Field,
    next_stone: Texture,
    current_stone: Stone,
    time_per_tick: Duration,
    time_until_next_tick: Duration,
    points: u64,
    ticks: u32,
    level: u8
}

impl Default for PlayView {
    fn default() -> Self {
        let first_block_texture = Stone::new_random_texture();
        let first_block_position = Self::get_spawn_position(&first_block_texture.dimensions);

        Self {
            field: field::Field::default(),
            next_stone: Stone::new_random_texture(),
            current_stone: Stone::new(first_block_position, first_block_texture),
            time_until_next_tick: Duration::from_millis(1000),
            time_per_tick: Duration::from_millis(1000),
            ticks: 0,
            points: 0,
            level: 1
        }
    }
}

impl PlayView {
    pub fn render_at(&self, canvas: &mut Canvas, position: Position) {
        self.field.render_at(canvas, position);
        canvas.add_texture(&self.next_stone, &Position { x: 10, y: 0 });
        canvas.add_texture(&self.current_stone.texture, &self.current_stone.position);
        canvas.add_text(format!("level: {}", self.level).as_str(), Color::Black, Color::Orange, &Position {
            x: 10, y: 6
        });
        canvas.add_text(format!("points: {}", self.points).as_str(), Color::Black, Color::Orange, &Position {
            x: 10, y: 7
        });
        //TODO add text to canvas
//        canvas.extend_from_slice(cursor::Goto(1, 21).to_string().as_bytes());
//        canvas.extend_from_slice("q - quit\r\nesq - menu\r\narrows - move block\r\n".as_bytes());
    }

    fn spawn_next_stone(&mut self) {
        self.field.add_to_texture(self.current_stone.texture.clone(), self.current_stone.position);

        let new_stone = Stone::new(
            Self::get_spawn_position(&self.next_stone.dimensions),
            self.next_stone.clone(),
        );

        self.current_stone = new_stone;
        self.next_stone = Stone::new_random_texture();
    }

    fn get_spawn_position(dimensions: &Dimensions) -> Position {
        Position {
            x: 5 - (dimensions.width as i8) / 2,
            y: -(dimensions.height as i8) + 1,
        }
    }

    pub fn handle_input(&mut self, event: &crossterm::event::Event) {
        use crossterm::event::{Event, KeyEvent, KeyCode};
        match event {
            Event::Key(KeyEvent { code, modifiers: _ }) => {
                match code {
                    KeyCode::Left => self.current_stone.move_left(&self.field),
                    KeyCode::Right => self.current_stone.move_right(&self.field),
                    KeyCode::Down => self.current_stone.move_down(&self.field),
                    KeyCode::Up => self.current_stone.rotate(),
                    KeyCode::Char(' ') => {
                        self.spawn_next_stone();
                        false
                    }
                    _ => false
                }
            }
            _ => false
        };
    }

    fn update_score(&mut self, deleted_lines: usize) {
        if deleted_lines == 0 {
            return;
        }

        let action_score = 2u8.pow(deleted_lines as u32) as u64 * self.level as u64;
        self.points += action_score;
        self.level = 1 + (self.points / 10) as u8;
    }

    fn handle_tick(&mut self) {
        if !self.current_stone.move_down(&self.field) {
            self.spawn_next_stone();
            let deleted_lines = self.field.try_delete_lines();
            self.update_score(deleted_lines);
        }
    }

    async fn wait_for_next_tick(game_state: Arc<Mutex<Game>>) {
        let time_until_next_tick = async {
            let game = game_state.lock().await;
            game.current_view.time_until_next_tick
        }.await;

        delay_for(time_until_next_tick).await;
    }

    async fn progress_game(game_state: Arc<Mutex<Game>>) {
        let mut game = game_state.lock().await;
        let play_view = &mut game.current_view;
        play_view.handle_tick();
        play_view.ticks += 1;
        play_view.time_per_tick = Duration::from_millis(std::cmp::max(100, (play_view.time_per_tick.as_millis() as u64 - 5).into()));
        play_view.time_until_next_tick = play_view.time_per_tick;
    }

    pub async fn generate_ticks(game_state: Arc<Mutex<Game>>) {
        loop {
            Self::wait_for_next_tick(game_state.clone()).await;
            Self::progress_game(game_state.clone()).await;
        }
    }
}
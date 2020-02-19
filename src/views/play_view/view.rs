use super::field;
use super::stones::Stone;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use crate::game::Game;
use tokio::time::delay_for;
use crate::rendering::renderer::{Texture, Position, Canvas, Dimensions};
use tokio::task::JoinHandle;

pub struct PlayView {
    field: field::Field,
    next_stone: Texture,
    current_stone: Stone,
    time_per_tick: Duration,
    time_until_next_tick: Duration,
    points: u64,
    cleared_lines: u64,
    level: u8,
    minimal_tick_time: Duration,
    tick_generator: Option<JoinHandle<()>>
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
            points: 0,
            level: 1,
            cleared_lines: 0,
            minimal_tick_time: Duration::from_millis(30),
            tick_generator: None
        }
    }
}

impl PlayView {
    pub fn render_at(&self, canvas: &mut Canvas, position: Position) {
        self.field.render_at(canvas, position);
        canvas.add_texture(self.next_stone.clone(), &Position { x: 10, y: 0 });
        canvas.add_texture(self.current_stone.texture.clone(), &self.current_stone.position);
        canvas.add_themed_text(format!("level: {}", self.level).as_str(), &Position { x: 10, y: 6 });
        canvas.add_themed_text(format!("points: {}", self.points).as_str(), &Position { x: 10, y: 7 });
        canvas.add_themed_paragraph(vec!["q - quit", "esq - menu", "arrows - move block"].as_slice(), Position { x: 0, y: 21 });
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
                    KeyCode::Up => self.current_stone.rotate(&self.field),
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

        self.cleared_lines += deleted_lines as u64;
        let action_score = 2u8.pow(deleted_lines as u32) as u64 * self.level as u64;
        self.points += action_score;
        self.level = 1u8 + (self.cleared_lines / 10) as u8;
    }

    fn cancel_tick_generator(&mut self) {
        //TODO implement cancellation of tick generator, dropping JoinHandle doesn't work
        self.tick_generator = None;
    }

    fn stop_game(&mut self) {
        self.cancel_tick_generator();
    }

    fn progress_game(&mut self) {
        if !self.current_stone.move_down(&self.field) {
            if self.current_stone.position.y < 0 {
                self.stop_game();
            } else {
                self.spawn_next_stone();
                let deleted_lines = self.field.try_delete_lines();
                self.update_score(deleted_lines);
            }
        }
    }

    async fn wait_for_next_tick(game_state: Arc<Mutex<Game>>) {
        let time_until_next_tick = async {
            let game = game_state.lock().await;
            game.current_view.time_until_next_tick
        }.await;

        delay_for(time_until_next_tick).await;
    }

    async fn handle_tick(game_state: Arc<Mutex<Game>>) {
        let mut game = game_state.lock().await;
        let play_view = &mut game.current_view;
        play_view.progress_game();
        let proposed_tick_time = Duration::from_millis((1000.0 * 0.75f32.powi(play_view.level as i32)) as u64);
        play_view.time_per_tick = std::cmp::max(proposed_tick_time, play_view.minimal_tick_time);
        play_view.time_until_next_tick = play_view.time_per_tick;
    }

    pub async fn generate_ticks(game_state: Arc<Mutex<Game>>) {
        loop {
            Self::wait_for_next_tick(game_state.clone()).await;
            Self::handle_tick(game_state.clone()).await;
        }
    }

    pub fn on_create(&mut self, game_handle: Arc<Mutex<Game>>) {
        self.tick_generator = Some(tokio::spawn(Self::generate_ticks(game_handle)));
    }
}
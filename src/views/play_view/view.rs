use super::field;
use termion::{color, cursor};
use termion::color::Bg;
use tokio::io::{AsyncWriteExt, AsyncWrite};
use crate::renderer::{Texture, Position, render_at};
use super::stones::Stone;


pub struct PlayView {
    field: field::Field,
    next_stone: Texture,
    current_stone: Stone,
}

impl Default for PlayView {
    fn default() -> Self {
        let first_block_texture = Stone::new_random();
        let dimensions = first_block_texture.dimensions();
        let first_block_position = Position {
            x: (11 + dimensions.x as i8) / 2,
            y: -(dimensions.y as i8) + 1
        };


        Self {
            field: field::Field::default(),
            next_stone: Stone::new_random(),
            current_stone: Stone::new(first_block_position, first_block_texture),
        }
    }
}

impl PlayView {
    pub fn render_at(&self, canvas: &mut Vec<u8>, position: Position){
        self.field.render_at(canvas, position);
        render_at(canvas, Position{x: 10, y: 0}, &self.next_stone);
        render_at(canvas, self.current_stone.position, &self.current_stone.texture);
        canvas.extend_from_slice(cursor::Goto(1, 21).to_string().as_bytes());
        canvas.extend_from_slice("q - quit\r\nesq - menu\r\narrows - move block\r\n".as_bytes());
    }

    pub fn handle_input(&mut self, event: &crossterm::event::Event) {
        use crossterm::event::{Event, KeyEvent, KeyCode};
        match event {
            Event::Key(KeyEvent{code, modifiers: _}) => {
                match code {
                    KeyCode::Left => self.current_stone.position.move_left(),
                    KeyCode::Right => self.current_stone.position.move_right(),
                    KeyCode::Down => self.current_stone.position.move_down(),
                    KeyCode::Up => self.current_stone.position.move_up(),
                    _ => ()
                }
            }
            _ => ()
        };
    }
}
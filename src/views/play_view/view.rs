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
    pub async fn render_at<W: tokio::io::AsyncWrite + Unpin>(&self, term: &mut W, position: Position) -> tokio::io::Result<()>{
        self.field.render_at(term, position).await?;
        render_at(term, Position{x: 10, y: 0}, &self.next_stone).await?;
        render_at(term, self.current_stone.position, &self.current_stone.texture).await?;
        term.write_all(cursor::Goto(1, 21).to_string().as_bytes()).await?;
        term.write_all("q - quit\r\nesq - menu\r\narrows - move block\r\n".as_bytes()).await?;
//        println!("pos: {:#?}, dim: {:#?}", self.current_stone.position, self.current_stone.texture.dimensions());
        return Ok(())
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
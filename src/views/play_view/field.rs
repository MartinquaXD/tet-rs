use termion::{
    cursor,
    color:: {
        self,
        Bg
    }
};

use super::super::super::renderer::{render_at, Texture, Position};

pub struct Field(Texture);

impl Default for Field {
    fn default() -> Self {
        Self (Texture::new(10, 20))
    }
}

impl Field {
    pub fn render_at(&self, canvas: &mut Vec<u8>, position: Position){
        render_at(canvas, position, &self.0);
    }
}
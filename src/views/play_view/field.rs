use termion::{
    cursor,
    color:: {
        self,
        Bg
    }
};

use super::super::super::renderer::{Texture, Position};
use crate::renderer::Canvas;

pub struct Field(Texture);

impl Default for Field {
    fn default() -> Self {
        Self (Texture::new(10, 20))
    }
}

impl Field {
    pub fn render_at(&self, canvas: &mut Canvas, position: Position){
        canvas.add_texture( &self.0, position);
    }
}
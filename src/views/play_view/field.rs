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
    pub async fn render_at<W: tokio::io::AsyncWrite + Unpin>(&self, term: &mut W, position: Position) -> tokio::io::Result<()>{
        render_at(term, position, &self.0).await
    }
}
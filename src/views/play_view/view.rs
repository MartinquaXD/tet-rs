use super::field;
use super::stones::{Stone};
use termion::{color, cursor};
use termion::color::Bg;
use tokio::io::{AsyncWriteExt, AsyncWrite};


pub struct PlayView {
    field: field::Field,
    next_stone: Stone
}

impl Default for PlayView {
    fn default() -> Self {
        Self {
            field: field::Field::default(),
            next_stone: Stone::new_random()
        }
    }
}

impl PlayView {
    pub async fn render_at<W: AsyncWrite + Unpin>(&self, term: &mut W, x_target: u16, y_target: u16) -> tokio::io::Result<()>{
        let mut out = Vec::with_capacity(200);
        for (y, line) in self.field.0.iter().enumerate() {
            out.extend_from_slice(cursor::Goto(x_target + 1, y_target + y as u16 + 1).to_string().as_bytes());
            for (x, block) in line.iter().enumerate() {
                let rnd = x as u8 * y as u8;
                let color = match block {
                    None => color::Rgb(rnd, rnd, rnd),
                    Some(color) => color.to_rgb(),
                };

                out.extend_from_slice(Bg(color).to_string().as_bytes());
                out.extend_from_slice(" ".as_bytes());
            }
        }
        out.extend_from_slice(Bg(color::Reset).to_string().as_bytes());
        term.write_all(&out).await?;
        return Ok(())
    }
}
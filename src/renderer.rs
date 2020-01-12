use termion::color::Rgb;
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red,
    Yellow,
    Green,
    LightBlue,
    DarkBlue,
    Orange,
    Purple,
}

impl Color {
    pub fn to_rgb(&self) -> Rgb {
        use self::Color::*;

        match self {
            Red => Rgb(255, 0, 0),
            Yellow => Rgb(255, 247, 5),
            Green => Rgb(0, 255, 0),
            LightBlue => Rgb(0, 170, 255),
            DarkBlue => Rgb(15, 32, 189),
            Orange => Rgb(245, 167, 66),
            Purple => Rgb(125, 15, 189)
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Position {
    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }
}

#[derive(Debug, Clone)]
pub struct Dimensions { pub x: usize, pub y: usize }

#[derive(Debug, Clone)]
pub struct Texture(pub Vec<Vec<Option<Color>>>);

impl Texture {
    pub fn new(x: usize, y: usize) -> Self {
        Self(vec![vec![None; x]; y])
    }

    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            x: self.0[0].len(),
            y: self.0.len(),
        }
    }
}


pub async fn render_at<W: tokio::io::AsyncWrite + Unpin>(term: &mut W, position: Position, texture: &Texture) -> tokio::io::Result<()> {
    use tokio::io::AsyncWriteExt;
    use termion::{
        cursor,
        color::{
            self,
            Bg
        }
    };
    use std::cmp::max;

    let mut out = Vec::new();

    let (window_x, window_y) = crossterm::terminal::size().unwrap();
    let dimensions = texture.dimensions();

    let skip_rows = max(-position.y as isize, 0) as usize;
    let skip_columns = max(-position.x as isize,  0) as usize;

    for (y, line) in texture.0.iter().skip(skip_rows).enumerate() {
        if y > window_y as usize {
            break;
        }

        let y_in_terminal_coords = (position.y + 1 + y as i8 + skip_rows as i8) as u16;
        let x_in_terminal_coords = (position.x + 1 + skip_columns as i8) as u16;

        out.extend_from_slice(cursor::Goto(x_in_terminal_coords, y_in_terminal_coords).to_string().as_bytes());
        for (x, block) in line.iter().enumerate().skip(skip_columns) {
            if x > window_x as usize {
                break;
            }

            let color: Vec<u8> = match block {
                None => color::Reset.bg_str().into(),
                Some(color) => color.to_rgb().bg_string().into(),
            };

            out.extend_from_slice(color.as_slice());
            out.extend_from_slice(" ".as_bytes());
        }
    }
    out.extend_from_slice(Bg(color::Reset).to_string().as_bytes());
    term.write_all(&out).await?;
    return Ok(());
}
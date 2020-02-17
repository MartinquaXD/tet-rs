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
    White,
    Black,
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
            Purple => Rgb(125, 15, 189),
            White => Rgb(255, 255, 255),
            Black => Rgb(0, 0, 0)
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

#[derive(Debug, Clone, Copy)]
pub struct Dimensions { pub width: usize, pub height: usize }

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    foreground: Color,
    background: Color,
    text: char,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            text: ' ',
        }
    }
}

impl Tile {
    pub fn new_background(background: Color) -> Self {
        Tile { background, foreground: Color::White, text: ' ' }
    }

    pub fn to_printable_string(&self) -> String {
        format!("{}{}{}", self.background.to_rgb().bg_string(),
                self.foreground.to_rgb().fg_string(),
                self.text)
    }
}

#[derive(Debug, Clone)]
pub struct Texture(pub Vec<Vec<Option<Tile>>>);

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![None; width]; height])
    }

    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.0[0].len(),
            height: self.0.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub dimensions: Dimensions,
    rows: Vec<Vec<Tile>>,
}

impl Default for Canvas {
    fn default() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap();
        Self {
            dimensions: Dimensions { width: width as usize, height: height as usize },
            rows: vec![vec![Tile::default(); width as usize]; height as usize],
        }
    }
}

impl Canvas {
    pub fn clear(&mut self) {
        self.rows.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|tile| {
                *tile = Tile::default();
            });
        });
    }

    pub fn to_printable_string(&self) -> String {
        //TODO create buffer with correct size
        let mut size = self.dimensions.width * self.dimensions.height * 34;
        let mut res = String::with_capacity(size);
        res.push_str(termion::cursor::Goto(1, 1).to_string().as_str());
        self.rows.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                res.push_str(tile.to_printable_string().as_str());
            });
//            res.push_str("\n\r");
        });
        res
    }

    pub fn add_texture(&mut self, texture: &Texture, position: Position) {
        use tokio::io::AsyncWriteExt;
        use termion::{
            cursor,
            color::{
                self,
                Bg,
            },
        };

        use std::cmp::{max, min};

        let texture_dimensions = texture.dimensions();

        let start_row_canvas = position.y.clamp(0, self.dimensions.height as i8) as usize;
        let start_column_canvas = position.x.clamp(0, self.dimensions.width as i8) as usize;

        let start_row_texture = min(max(-position.y as isize, 0) as usize, texture_dimensions.height);
        let start_column_texture = min(max(-position.x as isize, 0) as usize, texture_dimensions.width);

        //TODO fix slices out of bounds exception
        self.rows[start_row_canvas..].iter_mut().zip(texture.0[start_row_texture..].iter()).for_each(|(canvas_row, texture_row)| {
            canvas_row[start_column_canvas..].iter_mut().zip(texture_row[start_column_texture..].iter()).for_each(|(canvas_color, texture_color)| {
                if let Some(tile) = texture_color {
                    *canvas_color = tile.clone();
                }
            });
        });
    }
}
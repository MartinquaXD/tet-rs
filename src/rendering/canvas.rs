use crate::rendering::texture::{Dimensions, Texture};
use crate::rendering::tile::Tile;
use crate::rendering::position::Position;
use crate::rendering::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub dimensions: Dimensions,
    rows: Vec<Vec<Tile>>,
    buffer: String,
}

impl Default for Canvas {
    fn default() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap();
        Self {
            dimensions: Dimensions { width: width as usize, height: height as usize },
            rows: vec![vec![Tile::default(); width as usize]; height as usize],
            buffer: String::with_capacity(20000),
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

    pub fn get_printable_string(&mut self) -> &String {
        let mut buffer = std::mem::replace(&mut self.buffer, String::new());
        buffer.clear();
        buffer.push_str(crossterm::cursor::MoveTo(0, 0).to_string().as_str());

        let mut previous_tile = &Tile::default();
        previous_tile.fill_buffer_with_printable_string(&mut buffer);

        self.rows.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                tile.fill_buffer_with_printable_string_with_respect_to_previous_tile(&mut buffer, &previous_tile);
                previous_tile = tile;
            });
        });
        self.buffer = buffer;
        &self.buffer
    }

    pub fn add_themed_paragraph(&mut self, text: &[&str], mut position: Position) {
        text.iter().for_each(|text| {
            self.add_themed_text(text, &position);
            position.move_down();
        })
    }

    pub fn add_themed_text(&mut self, text: &str, position: &Position) {
        self.add_text(text, Color::Black, Color::Orange, position);
    }

    pub fn add_text(&mut self, text: &str, background: Color, foreground: Color, position: &Position) {
        let texture_to_draw: Vec<_> = text.chars().map(|letter| {
            Some(Tile::new_character(letter, background, foreground))
        }).collect();

        let texture = Texture {
            pixels: vec![texture_to_draw],
            dimensions: Dimensions { width: text.len(), height: 1 },
        };

        self.add_texture(texture, position);
    }

    pub fn add_texture(&mut self, texture: Texture, position: &Position) {
        use std::cmp::{max, min};

        let texture_dimensions = texture.dimensions;

        let start_row_canvas = position.y.clamp(0, self.dimensions.height as i8) as usize;
        let start_column_canvas = position.x.clamp(0, self.dimensions.width as i8) as usize;

        let start_row_texture = min(max(-position.y as isize, 0) as usize, texture_dimensions.height);
        let start_column_texture = min(max(-position.x as isize, 0) as usize, texture_dimensions.width);

        let texture_row_iterator = texture.pixels.into_iter().skip(start_row_texture);

        self.rows.iter_mut().skip(start_row_canvas).zip(texture_row_iterator)
            .for_each(|(canvas_row, texture_row)| {
                let texture_column_iterator = texture_row.into_iter().skip(start_column_texture);

                canvas_row.iter_mut().skip(start_column_canvas).zip(texture_column_iterator)
                    .for_each(|(canvas_color, texture_color)| {
                        if let Some(tile) = texture_color {
                            *canvas_color = tile;
                        }
                    });
            });
    }
}
use termion::{
    cursor,
    color::{
        self,
        Bg,
    },
};

use super::super::super::renderer::{Texture, Position};
use crate::renderer::{Canvas, Dimensions, Color, Tile};

pub struct Field {
    pub texture: Texture
}

impl Default for Field {
    fn default() -> Self {
        Self {
            texture: Texture::new_background(Dimensions { width: 10, height: 20 }, Color::Gray)
        }
    }
}

impl Field {
    pub fn render_at(&self, canvas: &mut Canvas, position: Position) {
        canvas.add_texture(&self.texture, position);
    }

    pub fn highest_point_in_column(&self, column: usize) -> Option<usize> {
        if column > self.texture.dimensions.width {
            return None;
        }

        let mut result = 0usize;
        for (index, row) in self.texture.pixels.iter().enumerate() {
            if let Some(Some(tile)) = row.get(column) {
                result = self.texture.dimensions.height - index;
                break;
            }
        }

        Some(result)
    }

    pub fn get_tile_at_pos(&self, position: &Position) -> Option<&Tile> {
        if let Some(row) = self.texture.pixels.get(position.y as usize) {
            if let Some(Some(tile)) = row.get(position.x as usize) {
                return Some(tile);
            }
        }

        return None;
    }

    pub fn get_tile_at_pos_mut(&mut self, position: &Position) -> Option<&mut Tile> {
        if let Some(row) = self.texture.pixels.get_mut(position.y as usize) {
            if let Some(Some(tile)) = row.get_mut(position.x as usize) {
                return Some(tile);
            }
        }

        return None;
    }

    pub fn all_positions_free(&self, positions: &[Position]) -> bool {
        positions.iter().all(|position| {
            if let Some(tile) = self.get_tile_at_pos(position) {
                tile.background == Color::Gray
            } else {
                false
            }
        })
    }

    pub fn add_to_texture(&mut self, texture: Texture, position: Position) {
        for (row_index, row) in texture.pixels.iter().enumerate() {
            for (column_index, tile_to_add) in row.iter().enumerate() {
                if let Some(tile_to_add) = tile_to_add {
                    let pos = Position {
                        x: position.x + column_index as i8,
                        y: position.y + row_index as i8,
                    };

                    self.get_tile_at_pos_mut(&pos).map(|current_tile| {
                        *current_tile = *tile_to_add;
                    });
                }
            }
        }
    }
}
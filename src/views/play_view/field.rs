use crate::rendering::renderer::{Texture, Dimensions, Color, Position, Tile, Canvas};

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
        canvas.add_texture(self.texture.clone(), &position);
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
            match self.get_tile_at_pos(position) {
                Some(tile) => tile.background == Color::Gray,
                None => true
            }
        })
    }

    fn row_is_full(row: &Vec<Option<Tile>>) -> bool {
        row.iter().all(|tile| tile.as_ref().unwrap().background != Color::Gray)
    }

    fn clear_row(row: &mut Vec<Option<Tile>>) {
        row.iter_mut().for_each(|tile| tile.as_mut().unwrap().background = Color::Gray)
    }

    pub fn try_delete_lines(&mut self) -> usize {
        let mut empty_lines: Vec<_> = self.texture.pixels.drain_filter(|row |{
            if Self::row_is_full(row) {
                Self::clear_row(row);
                true
            } else {
                false
            }
        }).collect();

        let old_lines = std::mem::replace(&mut self.texture.pixels, vec![]);
        let lines_deleted = empty_lines.len();
        empty_lines.extend(old_lines);
        self.texture.pixels = empty_lines;
        lines_deleted
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
                        *current_tile = tile_to_add.clone();
                    });
                }
            }
        }
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.texture.dimensions
    }
}
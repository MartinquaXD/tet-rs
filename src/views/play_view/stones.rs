use rand::{thread_rng, Rng};
use crate::views::play_view::field::Field;
use crate::rendering::renderer::{Texture, Position, Canvas, Tile, Color::*, Dimensions};

#[derive(Debug, Clone)]
pub struct Stone {
    pub texture: Texture,
    pub position: Position,
}

impl Stone {
    pub fn new(position: Position, texture: Texture) -> Self {
        Self {
            position,
            texture,
        }
    }

    pub fn render_at(&self, canvas: &mut Canvas) {
        canvas.add_texture(self.texture.clone(), &self.position);
    }

    pub fn new_i() -> Texture {
        Texture {
            pixels: vec![
                vec![Some(Tile::new_background(LightBlue))],
                vec![Some(Tile::new_background(LightBlue))],
                vec![Some(Tile::new_background(LightBlue))],
                vec![Some(Tile::new_background(LightBlue))],
            ],
            dimensions: Dimensions { width: 1, height: 4 },
        }
    }

    pub fn new_z() -> Texture {
        Texture {
            pixels: vec![
                vec![Some(Tile::new_background(Red)), Some(Tile::new_background(Red)), None],
                vec![None, Some(Tile::new_background(Red)), Some(Tile::new_background(Red))],
            ],
            dimensions: Dimensions { width: 3, height: 2 },
        }
    }

    pub fn new_s() -> Texture {
        Texture {
            pixels: vec![
                vec![None, Some(Tile::new_background(Green)), Some(Tile::new_background(Green))],
                vec![Some(Tile::new_background(Green)), Some(Tile::new_background(Green)), None],
            ],
            dimensions: Dimensions { width: 3, height: 2 },
        }
    }

    pub fn new_j() -> Texture {
        Texture {
            pixels: vec![
                vec![Some(Tile::new_background(DarkBlue)), None, None],
                vec![Some(Tile::new_background(DarkBlue)), Some(Tile::new_background(DarkBlue)), Some(Tile::new_background(DarkBlue))],
            ],
            dimensions: Dimensions { width: 3, height: 2 },
        }
    }

    pub fn new_l() -> Texture {
        Texture {
            pixels: vec![
                vec![None, None, Some(Tile::new_background(Orange))],
                vec![Some(Tile::new_background(Orange)), Some(Tile::new_background(Orange)), Some(Tile::new_background(Orange))],
            ],
            dimensions: Dimensions { width: 3, height: 2 },
        }
    }

    pub fn new_o() -> Texture {
        Texture {
            pixels: vec![
                vec![Some(Tile::new_background(Yellow)), Some(Tile::new_background(Yellow))],
                vec![Some(Tile::new_background(Yellow)), Some(Tile::new_background(Yellow))],
            ],
            dimensions: Dimensions { width: 2, height: 2 },
        }
    }

    pub fn new_t() -> Texture {
        Texture {
            pixels: vec![
                vec![None, Some(Tile::new_background(Purple)), None],
                vec![Some(Tile::new_background(Purple)), Some(Tile::new_background(Purple)), Some(Tile::new_background(Purple))],
            ],
            dimensions: Dimensions { width: 3, height: 2 },
        }
    }

    pub fn new_random_texture() -> Texture {
        let mut rng = thread_rng();

        match rng.gen_range(0, 7) {
            0 => Self::new_i(),
            1 => Self::new_j(),
            2 => Self::new_l(),
            3 => Self::new_z(),
            4 => Self::new_s(),
            5 => Self::new_o(),
            6 => Self::new_t(),
            _ => panic!("this value should not have been generated")
        }
    }

    fn left_most_points(&self) -> Vec<Position> {
        self.texture.pixels.iter().enumerate().map(|(row_number, columns)| {
            let left_most = columns.iter().enumerate().find_map(|(index, tile)| {
                if tile.is_some() { Some(index as i8) } else { None }
            });

            Position {
                x: self.position.x + left_most.unwrap(),
                y: self.position.y + row_number as i8,
            }
        }).collect()
    }



    fn right_most_points(&self) -> Vec<Position> {
        self.texture.pixels.iter().enumerate().map(|(row_number, columns)| {
            let right_most = columns.iter().enumerate().rev().find_map(|(index, tile)| {
                if tile.is_some() { Some(index as i8) } else { None }
            });

            Position {
                x: self.position.x + right_most.unwrap(),
                y: self.position.y + row_number as i8,
            }
        }).collect()
    }

    fn bottom_points(&self) -> Vec<Position> {
        self.texture.pixels.iter().enumerate().fold(vec![Position::default(); self.texture.dimensions.width],
            |mut acc, (row_index, row)|{
                for (column_index, tile) in row.iter().enumerate() {
                    if tile.is_some() {
                        *acc.get_mut(column_index).unwrap() = Position {
                            x: self.position.x + column_index as i8,
                            y: self.position.y + row_index as i8
                        }
                    }
                }

                acc
        })
    }

    fn can_move_left(&self, field: &Field) -> bool {
        let not_on_left_border = self.position.x > 0;
        let mut positions_to_left = self.left_most_points();
        positions_to_left.iter_mut().for_each(|position| position.move_left());
        not_on_left_border && field.all_positions_free(positions_to_left.as_slice())
    }

    fn can_move_right(&self, field: &Field) -> bool {
        let not_on_right_border = self.position.x as usize + self.texture.dimensions.width < field.texture.dimensions.width;
        let mut positions_to_right = self.right_most_points();
        positions_to_right.iter_mut().for_each(|position| position.move_right());
        not_on_right_border && field.all_positions_free(positions_to_right.as_slice())
    }

    fn can_move_down(&self, field: &Field) -> bool {
        let not_at_bottom = (self.position.y + self.texture.dimensions.height as i8) < 20;
        let mut bottom_positions = self.bottom_points();
        bottom_positions.iter_mut().for_each(|position| position.move_down());
        not_at_bottom && field.all_positions_free(bottom_positions.as_slice())
    }

    pub fn move_down(&mut self, field: &Field) -> bool {
        if self.can_move_down(&field) {
            self.position.move_down();
            true
        } else {
            false
        }
    }

    pub fn move_left(&mut self, field: &Field) -> bool {
        if self.can_move_left(&field) {
            self.position.move_left();
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self, field: &Field) -> bool {
        if self.can_move_right(&field) {
            self.position.move_right();
            true
        } else {
            false
        }
    }

    fn would_be_in_bounds_after_rotation(&self, field: &Field) -> bool {
        let new_dimensions = self.texture.dimensions.transpose_into();
        let container = &field.texture.dimensions;

        self.position.x + (new_dimensions.width as i8) <= container.width as i8 &&
            self.position.y + (new_dimensions.height as i8) <= container.height as i8
    }

    pub fn rotate(&mut self, field: &Field) -> bool {
        if self.would_be_in_bounds_after_rotation(field) {
            self.texture.rotate();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::views::play_view::stones::Stone;
    use crate::rendering::renderer::Position;

    #[test]
    fn right_points() {
        let mut t_stone = Stone::new(Position{x: 2, y: -1}, Stone::new_t());
        t_stone.texture.rotate();
        let right_points = t_stone.right_most_points();
        let expected = vec![Position{x: 2, y: -1}, Position{x: 3, y: 0}, Position{x: 2, y: 1}];
        assert_eq!(expected, right_points);
    }

    #[test]
    fn left_points() {
        let mut t_stone = Stone::new(Position{x: 2, y: -1}, Stone::new_t());
        t_stone.texture.rotate();
        t_stone.texture.rotate();
        t_stone.texture.rotate();
        let left_points = t_stone.left_most_points();
        let expected = vec![Position{x: 3, y: -1}, Position{x: 2, y: 0}, Position{x: 3, y: 1}];
        assert_eq!(expected, left_points);
    }

    #[test]
    fn bottom_points() {
        let mut t_stone = Stone::new(Position{x: 2, y: -1}, Stone::new_t());
        t_stone.texture.rotate();
        t_stone.texture.rotate();
        let bottom_points = t_stone.bottom_points();
        let expected = vec![Position{x: 2, y: -1}, Position{x: 3, y: 0}, Position{x: 4, y: -1}];
        assert_eq!(expected, bottom_points);
    }
}
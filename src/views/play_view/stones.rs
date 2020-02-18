use crate::renderer::{Texture, Position, Color::{self, *}, Tile, Canvas, Dimensions};
use rand::{thread_rng, Rng};
use crate::views::play_view::field::Field;

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
        canvas.add_texture(&self.texture, self.position);
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

//        return Self::new_o();

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
                if tile.is_some() {
                    Some(index)
                } else {
                    None
                }
            });

            Position {
                x: left_most.unwrap() as i8 + self.position.x,
                y: self.position.y + row_number as i8,
            }
        }).collect()
    }

    fn right_most_points(&self) -> Vec<Position> {
        self.texture.pixels.iter().enumerate().map(|(row_number, columns)| {
            let right_most = columns.iter().enumerate().rev().find_map(|(index, tile)| {
                if tile.is_some() {
                    Some(index)
                } else {
                    None
                }
            });

            Position {
                x: right_most.unwrap() as i8 + self.position.x,
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
        let mut fields_to_the_left = self.left_most_points();
        fields_to_the_left.iter_mut().for_each(|pos| {
            pos.x -= 1;
        });
        not_on_left_border && field.all_positions_free(fields_to_the_left.as_slice())
    }

    fn can_move_right(&self, field: &Field) -> bool {
        let not_on_right_border = self.position.x as usize + self.texture.dimensions.width <= field.texture.dimensions.width;
        let mut fields_to_the_right = self.right_most_points();
        fields_to_the_right.iter_mut().for_each(|pos| {
            pos.x += 1;
        });
        not_on_right_border && field.all_positions_free(fields_to_the_right.as_slice())
    }

    fn can_move_down(&mut self, field: &Field) -> bool {
        let not_at_bottom = (self.position.y + self.texture.dimensions.height as i8) < 20;
        let mut bottom_fields = self.bottom_points();
        bottom_fields.iter_mut().for_each(|pos| {
            pos.y += 1;
        });
        not_at_bottom && field.all_positions_free(bottom_fields.as_slice())
    }

    pub fn move_down(&mut self, field: &Field) -> bool {
        let allow_move = self.can_move_down(&field);
        if allow_move {
            self.position.move_down();
        }
        allow_move
    }

    pub fn move_left(&mut self, field: &Field) -> bool {
        let allow_move = self.can_move_left(&field);
        if allow_move {
            self.position.move_left();
        }
        allow_move
    }

    pub fn move_right(&mut self, field: &Field) -> bool {
        let allow_move = self.can_move_right(&field);
        if allow_move {
            self.position.move_right();
        }
        allow_move
    }

    pub fn move_up(&mut self, field: &Field) -> bool {
        if true {
            self.position.move_up();
        }

        false
    }
}
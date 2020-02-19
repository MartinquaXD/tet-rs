use crate::rendering::tile::Tile;
use crate::rendering::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions { pub width: usize, pub height: usize }

impl Dimensions {
    pub fn transpose(&mut self){
        let temp = self.width;
        self.width = self.height;
        self.height = temp;
    }

    pub fn transpose_into(&self) -> Self{
        Self {
            width: self.height,
            height: self.width
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub pixels: Vec<Vec<Option<Tile>>>,
    pub dimensions: Dimensions,
}

impl Texture {
    pub fn new(dimensions: Dimensions) -> Self {
        Self {
            pixels: vec![vec![None; dimensions.width]; dimensions.height],
            dimensions,
        }
    }

    pub fn new_background(dimensions: Dimensions, background: Color) -> Self {
        Self {
            pixels: vec![vec![Some(Tile::new_background(background)); dimensions.width]; dimensions.height],
            dimensions,
        }
    }

    pub fn rotate(&mut self) {
        let old_data = std::mem::replace(&mut self.pixels, vec![vec![None; self.dimensions.height]; self.dimensions.width]);

        for (row_index, row) in old_data.into_iter().rev().enumerate() {
            for (column_index, tile) in row.into_iter().enumerate() {
                let other = self.pixels.get_mut(column_index).unwrap().get_mut(row_index).unwrap();
                *other = tile;
            }
        }

        self.dimensions.transpose();
    }
}
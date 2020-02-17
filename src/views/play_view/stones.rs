use crate::renderer::{Texture, Position, Color::{self, *}, Tile, Canvas};
use rand::{thread_rng, Rng};

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
        Texture(vec![
            vec![Some(Tile::new_background(LightBlue))],
            vec![Some(Tile::new_background(LightBlue))],
            vec![Some(Tile::new_background(LightBlue))],
            vec![Some(Tile::new_background(LightBlue))],
        ])
    }

    pub fn new_z() -> Texture {
        Texture(vec![
            vec![Some(Tile::new_background(Red)), Some(Tile::new_background(Red)), None],
            vec![None, Some(Tile::new_background(Red)), Some(Tile::new_background(Red))],
        ])
    }

    pub fn new_s() -> Texture {
        Texture(vec![
            vec![None, Some(Tile::new_background(Green)), Some(Tile::new_background(Green))],
            vec![Some(Tile::new_background(Green)), Some(Tile::new_background(Green)), None],
        ])
    }

    pub fn new_j() -> Texture {
        Texture(vec![
            vec![Some(Tile::new_background(DarkBlue)), None, None],
            vec![Some(Tile::new_background(DarkBlue)), Some(Tile::new_background(DarkBlue)), Some(Tile::new_background(DarkBlue))],
        ])
    }

    pub fn new_l() -> Texture {
        Texture(vec![
            vec![None, None, Some(Tile::new_background(Orange))],
            vec![Some(Tile::new_background(Orange)), Some(Tile::new_background(Orange)), Some(Tile::new_background(Orange))],
        ])
    }

    pub fn new_o() -> Texture {
        Texture(vec![
            vec![Some(Tile::new_background(Yellow)), Some(Tile::new_background(Yellow))],
            vec![Some(Tile::new_background(Yellow)), Some(Tile::new_background(Yellow))],
        ])
    }

    pub fn new_t() -> Texture {
        Texture(vec![
            vec![None, Some(Tile::new_background(Purple)), None],
            vec![Some(Tile::new_background(Purple)), Some(Tile::new_background(Purple)), Some(Tile::new_background(Purple))],
        ])
    }

    pub fn new_random() -> Texture {
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
}
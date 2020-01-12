use crate::renderer::{Texture, Position, Color::{self, *}, render_at};
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

    pub fn render_at(&self, canvas: &mut Vec<u8>) {
        render_at(canvas, self.position, &self.texture);
    }

    pub fn new_i() -> Texture {
        Texture(vec![
            vec![Some(LightBlue)],
            vec![Some(LightBlue)],
            vec![Some(LightBlue)],
            vec![Some(LightBlue)],
        ])
    }

    pub fn new_z() -> Texture {
        Texture(vec![
            vec![Some(Red), Some(Red), None],
            vec![None, Some(Red), Some(Red)],
        ])
    }

    pub fn new_s() -> Texture {
        Texture(vec![
            vec![None, Some(Green), Some(Green)],
            vec![Some(Green), Some(Green), None],
        ])
    }

    pub fn new_j() -> Texture {
        Texture(vec![
            vec![Some(DarkBlue), None, None],
            vec![Some(DarkBlue), Some(DarkBlue), Some(DarkBlue)],
        ])
    }

    pub fn new_l() -> Texture {
        Texture(vec![
            vec![None, None, Some(Orange)],
            vec![Some(Orange), Some(Orange), Some(Orange)],
        ])
    }

    pub fn new_o() -> Texture {
        Texture(vec![
            vec![Some(Yellow), Some(Yellow)],
            vec![Some(Yellow), Some(Yellow)],
        ])
    }

    pub fn new_t() -> Texture {
        Texture(vec![
            vec![None, Some(Purple), None],
            vec![Some(Purple), Some(Purple), Some(Purple)],
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
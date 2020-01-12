use rand::{thread_rng, Rng};
use termion::color::Rgb;

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


#[derive(Debug, Clone)]
pub struct Stone(Vec<Vec<Option<Color>>>);

impl Stone{
    pub fn new_i() -> Self {
        use Color::LightBlue;
        Self ( vec![
                vec![Some(LightBlue)],
                vec![Some(LightBlue)],
                vec![Some(LightBlue)],
                vec![Some(LightBlue)],
            ]
        )
    }

    pub fn new_z() -> Self {
        use Color::Red;
        Self (vec![
            vec![Some(Red), Some(Red), None],
            vec![None, Some(Red), Some(Red)],
        ])
    }

    pub fn new_s() -> Self {
        use Color::Green;
        Self (vec![
            vec![None, Some(Green), Some(Green)],
            vec![Some(Green), Some(Green), None],
        ])
    }

    pub fn new_j() -> Self {
        use Color::DarkBlue;
        Self (vec![
            vec![Some(DarkBlue), None, None],
            vec![Some(DarkBlue), Some(DarkBlue), Some(DarkBlue)],
        ])
    }

    pub fn new_l() -> Self {
        use Color::Orange;
        Self (vec![
            vec![None, None, Some(Orange)],
            vec![Some(Orange), Some(Orange), Some(Orange)],
        ])
    }

    pub fn new_o() -> Self {
        use Color::Yellow;
        Self (vec![
            vec![Some(Yellow), Some(Yellow)],
            vec![Some(Yellow), Some(Yellow)],
        ])
    }

    pub fn new_t() -> Self {
        use Color::Purple;
        Self (vec![
            vec![None, Some(Purple), None],
            vec![Some(Purple), Some(Purple), Some(Purple)],
        ])
    }

    pub fn new_random() -> Self {
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

    //TODO rotate_left, rotate_right
}
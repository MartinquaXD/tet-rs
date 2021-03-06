use crossterm::style::Color as Term_Color;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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
    Gray,
}

impl Color {
    pub fn to_rgb(&self) -> Term_Color {
        use Term_Color::*;


        match self {
            Color::Red => Rgb{r: 255, g: 0, b: 0},
            Color::Yellow => Rgb{r: 255, g: 247, b: 5},
            Color::Green => Rgb{r: 0, g: 255, b: 0},
            Color::LightBlue => Rgb{r: 0, g: 170, b: 255},
            Color::DarkBlue => Rgb{r: 15, g: 32, b: 189},
            Color::Orange => Rgb{r: 245, g: 167, b: 66},
            Color::Purple => Rgb{r: 125, g: 15, b: 189},
            Color::White => Rgb{r: 255, g: 255, b: 255},
            Color::Black => Rgb{r: 0, g: 0, b: 0},
            Color::Gray => Rgb{r: 100, g: 100, b: 100}
        }
    }

    pub fn to_ansi(&self) -> &str {
        match self {
            Color::Red => "255;0;0",
            Color::Yellow => "255;247;5",
            Color::Green => "0;255;0",
            Color::LightBlue => "0;170;255",
            Color::DarkBlue => "15;32;189",
            Color::Orange => "245;167;66",
            Color::Purple => "125;15;189",
            Color::White => "255;255;255",
            Color::Black => "0;0;0",
            Color::Gray => "100;100;100"
        }
    }
}
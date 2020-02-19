use crate::rendering::color::Color;

#[derive(Clone, Debug)]
pub struct Tile {
    pub foreground: Color,
    pub background: Color,
    pub text: char,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
            text: ' ',
        }
    }
}

impl Tile {
    pub fn new_background(background: Color) -> Self {
        Tile { background, foreground: Color::White, text: ' ' }
    }

    pub fn new_character(text: char, background: Color, foreground: Color) -> Self {
        Tile { background, foreground, text }
    }

    pub fn fill_buffer_with_printable_string(&self, buffer: &mut String) {
        buffer.push_str("\u{1b}[48;2;");
        buffer.push_str(self.background.to_ansi());
        buffer.push_str(";38;2;");
        buffer.push_str(self.foreground.to_ansi());
        buffer.push('m');
        buffer.push(self.text);
    }

    pub fn to_printable_string(&self) -> String {
        use crossterm::style::{SetBackgroundColor, SetForegroundColor};
        format!("{}{}{}", SetBackgroundColor(self.background.to_rgb()).to_string().as_str(),
                SetForegroundColor(self.foreground.to_rgb()).to_string(),
                self.text)
    }
}

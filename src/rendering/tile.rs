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

    fn push_background(&self, buffer: &mut String) {
        buffer.push_str("\u{1b}[48;2;");
        buffer.push_str(self.background.to_ansi());
        buffer.push('m');
    }

    fn push_foreground(&self, buffer: &mut String) {
        buffer.push_str("\u{1b}[38;2;");
        buffer.push_str(self.foreground.to_ansi());
        buffer.push('m');
    }

    pub fn fill_buffer_with_printable_string(&self, buffer: &mut String) {
        buffer.push_str("\u{1b}[48;2;");
        buffer.push_str(self.background.to_ansi());
        buffer.push_str(";38;2;");
        buffer.push_str(self.foreground.to_ansi());
        buffer.push('m');
        buffer.push(self.text);
    }

    pub fn fill_buffer_with_printable_string_with_respect_to_previous_tile(&self, buffer: &mut String, previous_tile: &Tile) {
        if self.foreground != previous_tile.foreground {
            self.push_foreground(buffer);
        }

        if self.background != previous_tile.background {
            self.push_background(buffer);
        }

        buffer.push(self.text);
    }
}

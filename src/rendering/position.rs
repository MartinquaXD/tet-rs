#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}

impl Position {
    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }
}
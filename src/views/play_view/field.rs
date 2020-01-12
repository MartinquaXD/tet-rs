use super::stones::Color;

pub struct Field(
    pub [[Option<Color>; 20]; 10]
);

//impl Field {
    ///Executes one progression of the play_view.
    ///Moves blocks down and removes completed lines
//    pub fn advance(&mut self) -> Option<u8> {
//        None
//    }
//}

impl Default for Field {
    fn default() -> Self {
        Self([[None; 20]; 10])
    }
}

trait Drawable {
    fn render_at(x: usize, y: usize) -> ();
}
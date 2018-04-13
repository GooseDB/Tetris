use color::Color;

#[derive(Clone, Copy)]
pub enum Square {
    ThereIsNo,
    ThereIs(Color, Color),
}

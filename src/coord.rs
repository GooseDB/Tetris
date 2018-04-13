use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    pub const fn new(x: i32, y: i32) -> Self {
        Coord {
            x: x as f32,
            y: y as f32,
        }
    }
    pub const fn new_f(x: f32, y: f32) -> Self {
        Coord { x: x, y: y }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

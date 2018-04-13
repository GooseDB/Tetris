use shapes;
use color::Color;
use square::Square;
use consts::{BOOST_MULTIPLIER, FPS};
use consts::{GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH};
use consts::{BLOCK_SPEED_X, BLOCK_SPEED_Y, BLOCK_START_X, BLOCK_START_Y};
use coord::Coord;

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Shape {
    fn generate() -> Shape {
        let shapes = [
            Shape::I,
            Shape::J,
            Shape::L,
            Shape::O,
            Shape::S,
            Shape::T,
            Shape::Z,
        ];
        let mut rand = thread_rng();
        shapes[rand.gen_range(0, 7)]
    }
    pub fn points(&self) -> i32 {
        match *self {
            Shape::I => 4,
            Shape::J => 8,
            Shape::L => 8,
            Shape::O => 4,
            Shape::S => 16,
            Shape::T => 8,
            Shape::Z => 16,
        }
    }
    pub fn x(&self) -> f32 {
        match *self {
            Shape::I => 4.0,
            Shape::J => 3.0,
            Shape::L => 3.0,
            Shape::O => 2.0,
            Shape::S => 3.0,
            Shape::T => 3.0,
            Shape::Z => 3.0,
        }
    }
    pub fn y(&self) -> f32 {
        match *self {
            Shape::I => 1.0,
            Shape::J => 2.0,
            Shape::L => 2.0,
            Shape::O => 2.0,
            Shape::S => 2.0,
            Shape::T => 2.0,
            Shape::Z => 2.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    turn: i32,
    shape: Shape,
    speed: Coord,
    boosted: bool,
    position: Coord,
}

impl Block {
    pub fn new() -> Self {
        Block {
            turn: 0,
            speed: Coord::new(BLOCK_SPEED_X, BLOCK_SPEED_Y),
            shape: Shape::generate(),
            boosted: false,
            position: Coord::new(BLOCK_START_X, BLOCK_START_Y),
        }
    }
    pub fn position(&self) -> Coord {
        self.position
    }
    pub fn shape(&self) -> Shape {
        self.shape
    }
    pub fn left(&mut self, field: &[Square; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize]) {
        self.position.x -= self.speed.x;
        if self.detect_collision(field) {
            self.position.x += self.speed.x;
        }
    }
    pub fn right(&mut self, field: &[Square; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize]) {
        self.position.x += self.speed.x;
        if self.detect_collision(field) {
            self.position.x -= self.speed.x;
        }
    }
    pub fn up(&mut self) {
        if self.boosted {
            self.position.y -= self.speed.y * BOOST_MULTIPLIER / FPS;
        } else {
            self.position.y -= self.speed.y / FPS;
        }
    }
    pub fn lower(&mut self) {
        if self.boosted {
            self.position.y += self.speed.y * BOOST_MULTIPLIER / FPS;
        } else {
            self.position.y += self.speed.y / FPS;
        }
    }
    pub fn boost(&mut self) {
        self.boosted = true;
    }
    pub fn brake(&mut self) {
        self.boosted = false;
    }
    pub fn rotate(&mut self, field: &[Square; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize]) {
        self.turn += 1;
        if self.detect_collision(field) {
            self.turn -= 1;
        }
    }
    pub fn detect_collision(
        &mut self,
        field: &[Square; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize],
    ) -> bool {
        let mut result = false;
        let block = self.block().0;
        for i in 0..block.len() {
            let position = self.position + block[i];
            if position.y < 0.0 {
                continue;
            }
            if position.x as i32 >= 0 && (position.x as i32) < GAME_FIELD_WIDTH
                && (position.y as i32) < GAME_FIELD_HEIGHT
            {
                match field[(position.x as i32 + position.y as i32 * GAME_FIELD_WIDTH) as usize] {
                    Square::ThereIs(_, _) => {
                        result = true;
                        break;
                    }
                    Square::ThereIsNo => {}
                }
            } else {
                result = true;
                break;
            }
        }
        result
    }
    pub fn block(&self) -> ([Coord; 4], Color, Color) {
        let block: [Coord; 4];
        let color1: Color;
        let color2: Color;
        match self.shape {
            Shape::I => {
                let turn = self.turn % shapes::I.len() as i32;
                block = shapes::I[turn as usize];
                color1 = shapes::COLOR_I.0;
                color2 = shapes::COLOR_I.1;
            }
            Shape::J => {
                let turn = self.turn % shapes::J.len() as i32;
                block = shapes::J[turn as usize];
                color1 = shapes::COLOR_J.0;
                color2 = shapes::COLOR_J.1;
            }
            Shape::L => {
                let turn = self.turn % shapes::L.len() as i32;
                block = shapes::L[turn as usize];
                color1 = shapes::COLOR_L.0;
                color2 = shapes::COLOR_L.1;
            }
            Shape::O => {
                let turn = self.turn % shapes::O.len() as i32;
                block = shapes::O[turn as usize];
                color1 = shapes::COLOR_O.0;
                color2 = shapes::COLOR_O.1;
            }
            Shape::S => {
                let turn = self.turn % shapes::S.len() as i32;
                block = shapes::S[turn as usize];
                color1 = shapes::COLOR_S.0;
                color2 = shapes::COLOR_S.1;
            }
            Shape::T => {
                let turn = self.turn % shapes::T.len() as i32;
                block = shapes::T[turn as usize];
                color1 = shapes::COLOR_T.0;
                color2 = shapes::COLOR_T.1;
            }
            Shape::Z => {
                let turn = self.turn % shapes::Z.len() as i32;
                block = shapes::Z[turn as usize];
                color1 = shapes::COLOR_Z.0;
                color2 = shapes::COLOR_Z.1;
            }
        }
        (block, color1, color2)
    }
}

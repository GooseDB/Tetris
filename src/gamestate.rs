use ggez::event;
use block::Block;
use color::Color;
use coord::Coord;
use ggez::Context;
use ggez::graphics;
use square::Square;
use consts::SCORE_LINE;
use ggez::graphics::Font;
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use consts::{GAME_FIELD_POS_X, GAME_FIELD_POS_Y};
use consts::{GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH};
use consts::{GAME_FIELD_BORDER_SIZE, SQUARE_BORDER, SQUARE_SIDE};
use consts::{BACKGROUND_COLOR, GAME_FIELD_BORDER_COLOR, GAME_FIELD_COLOR};
use consts::{PREVIEW_POS_X, PREVIEW_POS_Y, PREVIEW_SIZE_X, PREVIEW_SIZE_Y};

pub struct Gamestate {
    score: i32,
    game_over: bool,
    next_block: Block,
    current_block: Block,
    field: [Square; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize],
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            score: 0,
            game_over: false,
            next_block: Block::new(),
            current_block: Block::new(),
            field: [Square::ThereIsNo; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize],
        }
    }
    pub fn input(&mut self, ctx: &mut Context, events: &mut event::Events, exit: &mut bool) {
        for event in events.poll() {
            ctx.process_event(&event);
            match event {
                event::Event::Quit { .. }
                | event::Event::KeyDown {
                    keycode: Some(event::Keycode::Escape),
                    ..
                } => *exit = true,
                event::Event::KeyDown {
                    keycode: Some(event::Keycode::Up),
                    ..
                } => self.current_block.rotate(&self.field),
                event::Event::KeyDown {
                    keycode: Some(event::Keycode::Right),
                    ..
                } => self.current_block.right(&self.field),
                event::Event::KeyDown {
                    keycode: Some(event::Keycode::Left),
                    ..
                } => self.current_block.left(&self.field),
                event::Event::KeyDown {
                    keycode: Some(event::Keycode::Down),
                    ..
                } => self.current_block.boost(),
                event::Event::KeyUp {
                    keycode: Some(event::Keycode::Down),
                    ..
                } => self.current_block.brake(),
                _ => {}
            }
        }
    }
    pub fn frame(&mut self) {
        self.current_block.lower();
        if self.current_block.detect_collision(&self.field) {
            if self.current_block.position().y as i32 <= 1 {
                self.game_over = true;
            }
            self.current_block.up();
            self.block_to_field();
            self.score += self.current_block.shape().points();
            self.current_block = self.next_block;
            self.next_block = Block::new();
            self.check_rows();
        }
    }
    pub fn draw(&self, ctx: &mut Context, font: &Font) {
        graphics::clear(ctx);
        self.draw_background(ctx);
        self.draw_field(ctx);
        self.draw_current_block(ctx);
        self.draw_preview(ctx);
        self.draw_next_block(ctx);
        self.draw_score(ctx, font);
        graphics::present(ctx);
    }
    fn draw_score(&self, ctx: &mut Context, font: &Font) {
        let text = graphics::Text::new(
            ctx,
            &("score: ".to_string() + &self.score.to_string()),
            &font,
        ).expect("!");
        graphics::draw(
            ctx,
            &text,
            graphics::Point2::new(
                PREVIEW_POS_X as f32,
                (PREVIEW_POS_Y + PREVIEW_SIZE_Y) as f32 - 40.0,
            ),
            0.0,
        ).expect("!");
    }
    fn block_to_field(&mut self) {
        let block = self.current_block.block();
        for i in 0..block.0.len() {
            if self.current_block.position().y as i32 <= 0 {
                continue;
            }
            let x = self.current_block.position().x as i32 + block.0[i].x as i32;
            let y = self.current_block.position().y as i32 + block.0[i].y as i32;
            self.field[(x + y * GAME_FIELD_WIDTH) as usize] = Square::ThereIs(block.1, block.2);
        }
    }
    fn check_rows(&mut self) {
        'y: for mut y in 0..GAME_FIELD_HEIGHT {
            for x in 0..GAME_FIELD_WIDTH {
                match self.field[(x + y * GAME_FIELD_WIDTH) as usize] {
                    Square::ThereIs(_, _) => {}
                    Square::ThereIsNo => continue 'y,
                }
            }
            self.score += SCORE_LINE;
            for row in 0..y {
                for col in 0..GAME_FIELD_WIDTH {
                    if row < 0 {
                        self.field[(col + (y - row) * GAME_FIELD_WIDTH) as usize] =
                            Square::ThereIsNo;
                    } else {
                        self.field[(col + (y - row) * GAME_FIELD_WIDTH) as usize] =
                            self.field[(col + (y - row - 1) * GAME_FIELD_WIDTH) as usize];
                    }
                }
            }
        }
    }
    fn draw_next_block(&self, ctx: &mut Context) {
        let block = self.next_block.block();
        let x = self.next_block.shape().x() / 2.0;
        let y = self.next_block.shape().y() / 2.0;
        for i in 0..4 {
            self.draw_square(
                ctx,
                Coord::new_f(
                    PREVIEW_POS_X as f32 + ((PREVIEW_SIZE_X as f32 * SQUARE_SIDE) / 2.0)
                        - x * SQUARE_SIDE + block.0[i].x * SQUARE_SIDE,
                    PREVIEW_POS_Y as f32 + ((PREVIEW_SIZE_Y as f32 * SQUARE_SIDE) / 2.0)
                        - y * SQUARE_SIDE + block.0[i].y * SQUARE_SIDE,
                ),
                block.1,
                block.2,
            );
        }
    }
    fn draw_preview(&self, ctx: &mut Context) {
        let preview = (
            Coord::new(PREVIEW_POS_X as i32, PREVIEW_POS_Y as i32),
            Coord::new(
                PREVIEW_SIZE_X * SQUARE_SIDE as i32,
                PREVIEW_SIZE_Y * SQUARE_SIDE as i32,
            ),
        );
        let rect = graphics::Rect::new(
            preview.0.x - GAME_FIELD_BORDER_SIZE,
            preview.0.y - GAME_FIELD_BORDER_SIZE,
            preview.1.x + GAME_FIELD_BORDER_SIZE * 2.0,
            preview.1.y + GAME_FIELD_BORDER_SIZE * 2.0,
        );
        graphics::set_color(
            ctx,
            graphics::Color::from_rgb(
                GAME_FIELD_BORDER_COLOR.red(),
                GAME_FIELD_BORDER_COLOR.green(),
                GAME_FIELD_BORDER_COLOR.blue(),
            ),
        ).expect("HA LOL");
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
        let rect = graphics::Rect::new(preview.0.x, preview.0.y, preview.1.x, preview.1.y);
        graphics::set_color(
            ctx,
            graphics::Color::from_rgb(
                GAME_FIELD_COLOR.0.red(),
                GAME_FIELD_COLOR.0.green(),
                GAME_FIELD_COLOR.0.blue(),
            ),
        ).expect("HA LOL");
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
    }
    fn draw_field(&self, ctx: &mut Context) {
        let field = (
            Coord::new(GAME_FIELD_POS_X as i32, GAME_FIELD_POS_Y as i32),
            Coord::new(
                GAME_FIELD_WIDTH * SQUARE_SIDE as i32,
                GAME_FIELD_HEIGHT * SQUARE_SIDE as i32,
            ),
        );
        let rect = graphics::Rect::new(
            field.0.x - GAME_FIELD_BORDER_SIZE,
            field.0.y - GAME_FIELD_BORDER_SIZE,
            field.1.x + GAME_FIELD_BORDER_SIZE * 2.0,
            field.1.y + GAME_FIELD_BORDER_SIZE * 2.0,
        );
        graphics::set_color(
            ctx,
            graphics::Color::from_rgb(
                GAME_FIELD_BORDER_COLOR.red(),
                GAME_FIELD_BORDER_COLOR.green(),
                GAME_FIELD_BORDER_COLOR.blue(),
            ),
        ).expect("HA LOL");
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
        for i in 0..GAME_FIELD_WIDTH {
            let rect = graphics::Rect::new(
                GAME_FIELD_POS_X as f32 + SQUARE_SIDE * i as f32,
                field.0.y,
                SQUARE_SIDE,
                field.1.y,
            );
            if i % 2 == 0 {
                graphics::set_color(
                    ctx,
                    graphics::Color::from_rgb(
                        GAME_FIELD_COLOR.0.red(),
                        GAME_FIELD_COLOR.0.green(),
                        GAME_FIELD_COLOR.0.blue(),
                    ),
                ).expect("HA LOL");
            } else {
                graphics::set_color(
                    ctx,
                    graphics::Color::from_rgb(
                        GAME_FIELD_COLOR.1.red(),
                        GAME_FIELD_COLOR.1.green(),
                        GAME_FIELD_COLOR.1.blue(),
                    ),
                ).expect("HA LOL");
            }
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
        }
        for i in 0..self.field.len() {
            let y = i as i32 / GAME_FIELD_WIDTH;
            let x = i as i32 % GAME_FIELD_WIDTH;
            match self.field[(x + y * GAME_FIELD_WIDTH) as usize] {
                Square::ThereIs(ref c1, ref c2) => self.draw_square(
                    ctx,
                    Coord::new(
                        GAME_FIELD_POS_X + x * SQUARE_SIDE as i32,
                        GAME_FIELD_POS_Y + y * SQUARE_SIDE as i32,
                    ),
                    *c1,
                    *c2,
                ),
                Square::ThereIsNo => {}
            }
        }
    }
    fn draw_background(&self, ctx: &mut Context) {
        let rect = graphics::Rect::new(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
        graphics::set_color(
            ctx,
            graphics::Color::from_rgb(
                BACKGROUND_COLOR.red(),
                BACKGROUND_COLOR.green(),
                BACKGROUND_COLOR.blue(),
            ),
        ).expect("HA LOL");
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
    }
    fn draw_current_block(&self, ctx: &mut Context) {
        let block = self.current_block.block();
        for i in 0..4 {
            self.draw_square(
                ctx,
                Coord::new(
                    GAME_FIELD_POS_X + (self.current_block.position().x as i32 * SQUARE_SIDE as i32)
                        + (block.0[i].x as i32 * SQUARE_SIDE as i32),
                    GAME_FIELD_POS_Y + (self.current_block.position().y as i32 * SQUARE_SIDE as i32)
                        + (block.0[i].y as i32 * SQUARE_SIDE as i32),
                ),
                block.1,
                block.2,
            );
        }
    }
    fn draw_square(&self, ctx: &mut Context, position: Coord, color1: Color, color2: Color) {
        if position.y as i32 <= 0 {
            return;
        }
        let rect = graphics::Rect::new(position.x, position.y, SQUARE_SIDE, SQUARE_SIDE);
        graphics::set_color(
            ctx,
            graphics::Color::from_rgb(color1.red(), color1.green(), color1.blue()),
        ).expect("HA LOL");
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
        let rect = graphics::Rect::new(
            position.x + SQUARE_BORDER,
            position.y + SQUARE_BORDER,
            SQUARE_SIDE - SQUARE_BORDER * 2.0,
            SQUARE_SIDE - SQUARE_BORDER * 2.0,
        );
        graphics::set_color(
            ctx,
            graphics::Color::from_rgb(color2.red(), color2.green(), color2.blue()),
        ).expect("HA LOL");
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).expect("BLEN");
    }
    pub fn game_over(&self) -> bool {
        self.game_over
    }
}

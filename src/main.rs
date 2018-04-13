#![feature(const_fn)]
#![feature(use_nested_groups)]
#![feature(duration_extras)]
extern crate ggez;
extern crate rand;

mod coord;
mod block;
mod color;
mod shapes;
mod consts;
mod square;
mod gamestate;

use ggez::event;
use ggez::Context;
use ggez::graphics;
use std::time::{Duration, Instant};
use std::thread::sleep;
use gamestate::Gamestate;
use consts::{FPS, NANOS_IN_SEC, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    let mut ctx = &mut ggez::ContextBuilder::new("eventloop", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("Failed to build ggez context");
    let mut events = event::Events::new(ctx).unwrap();
    let font = ggez::graphics::Font::new(ctx, "/3572.ttf", 15).expect("!");
    let mut exit = false;
    let mut now = Instant::now();
    let mut after = Instant::now();
    while !exit {
        let mut gamestate = Gamestate::new();
        while !gamestate.game_over() && !exit {
            gamestate.input(&mut ctx, &mut events, &mut exit);
            gamestate.frame();
            gamestate.draw(&mut ctx, &font);
            wait(&mut now, &mut after);
        }
        if !want_to_continue(&mut ctx, &mut events) {
            break;
        }
    }
}

fn wait(now: &mut Instant, after: &mut Instant) {
    *after = Instant::now();
    let frame = after.duration_since(*now).subsec_nanos();
    if NANOS_IN_SEC / FPS > frame as f32 {
        sleep(Duration::from_nanos(
            (NANOS_IN_SEC / FPS) as u64 - frame as u64,
        ));
    }
    *now = Instant::now();
}

fn want_to_continue(ctx: &mut Context, events: &mut event::Events) -> bool {
    graphics::set_color(ctx, graphics::Color::from_rgb(0, 0, 0)).expect("HA LOL");
    let font = ggez::graphics::Font::new(ctx, "/3572.ttf", 40).expect("!");
    let text = graphics::Text::new(ctx, &("GAME OVER".to_string()), &font).expect("!");
    graphics::draw(
        ctx,
        &text,
        graphics::Point2::new((WINDOW_WIDTH / 2 - 140) as f32, (WINDOW_HEIGHT / 2 - 30) as f32),
        0.0,
    ).expect("!");
    graphics::present(ctx);
    let mut ask = true;
    let mut answer = false;
    while ask {
        for event in events.poll() {
            ctx.process_event(&event);
            match event {
                event::Event::Quit { .. }
                | event::Event::KeyDown {
                    keycode: Some(event::Keycode::Escape),
                    ..
                } => {
                    answer = false;
                    ask = false;
                }
                event::Event::KeyDown {
                    keycode: Some(event::Keycode::Return),
                    ..
                } => {
                    answer = true;
                    ask = false;
                }
                _ => {}
            }
        }
    }
    answer
}

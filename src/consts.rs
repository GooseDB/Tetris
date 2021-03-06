use color::Color;

pub const FPS: f32 = 40.0;
pub const NANOS_IN_SEC: f32 = 1_000_000_000.0;
pub const GAME_FIELD_WIDTH: i32 = 10;
pub const GAME_FIELD_HEIGHT: i32 = 20;
pub const BLOCK_START_X: i32 = GAME_FIELD_WIDTH / 2 - 2;
pub const BLOCK_START_Y: i32 = -2;
pub const BLOCK_SPEED_Y: i32 = 4;
pub const BLOCK_SPEED_X: i32 = 1;
pub const WINDOW_WIDTH: u32 = 500;
pub const WINDOW_HEIGHT: u32 = 620;
pub const PREVIEW_POS_X: i32 = 320;
pub const PREVIEW_POS_Y: i32 = 250;
pub const PREVIEW_SIZE_X: i32 = 6;
pub const PREVIEW_SIZE_Y: i32 = 4;
pub const GAME_FIELD_POS_X: i32 = 20;
pub const GAME_FIELD_POS_Y: i32 = 20;
pub const GAME_FIELD_COLOR: (Color, Color) = (Color::new(168, 178, 147), Color::new(177, 187, 155));
pub const GAME_FIELD_BORDER_COLOR: Color = Color::new(135, 142, 117);
pub const GAME_FIELD_BORDER_SIZE: f32 = 5.0;
pub const BACKGROUND_COLOR: Color = Color::new(44, 56, 62);
pub const SQUARE_SIDE: f32 = 28.0;
pub const SQUARE_BORDER: f32 = 7.0;
pub const SCORE_LINE: i32 = 100;
pub const BOOST_MULTIPLIER: f32 = 4.0;

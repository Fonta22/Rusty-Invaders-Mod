use cgmath::Vector2;

pub const WINDOW_WIDTH: f64 = 1040.0;
pub const WINDOW_HEIGHT: f64 = 720.0;
pub const WINDOW_NAME: &str = "RustyInvaders";
pub const WINDOW_CENTER: Vector2<f64> = Vector2 {
  x: WINDOW_WIDTH / 2.0,
  y: WINDOW_HEIGHT / 2.0,
};

pub const PLAYER_INIT_X: f64 = WINDOW_CENTER.x;
pub const PLAYER_INIT_Y: f64 = WINDOW_HEIGHT - 90.0;
pub const PLAYER_WIDTH: f64 = 48.0;
pub const PLAYER_HEIGHT: f64 = 36.0;

pub const HEALTH_BAR_WIDTH: f64 = 160.0;
pub const HEALTH_BAR_HEIGHT: f64 = 16.0;
pub const HEALTH_BAR_STROKE: f64 = 4.0;
pub const HEALTH_REGEN_MAX_VALUE: f64 = 16.0;
pub const HEALTH_REGEN_MIN_VALUE: f64 = 8.0;

pub const ENEMY_WIDTH: f64 = 48.0;
pub const ENEMY_HEIGHT: f64 = 48.0;
pub const ENEMY_SPEED: f64 = 40.0;
pub const ENEMY_SPACING: f64 = 18.0; // 12 * 1.6 = 19.2 ~ 20.0
pub const ENEMY_MAX_DAMAGE: f64 = 15.0; // 12.0
pub const ENEMY_MIN_DAMAGE: f64 = 5.0; // 2.0
pub const ENEMY_GRID_COLS: u8 = 10;
pub const ENEMY_GRID_ROWS: u8 = 5;
pub const ENEMY_GRID_WIDTH: f64 = ENEMY_GRID_COLS as f64 * (ENEMY_WIDTH + ENEMY_SPACING);
pub const ENEMY_GRID_X: f64 = (WINDOW_WIDTH - (ENEMY_GRID_WIDTH)) / 2.0;
pub const ENEMY_GRID_Y: f64 = 16.0;

pub const ENEMY_OCTOPUS_POINTS: u64 = 10;
pub const ENEMY_CRAB_POINTS: u64 = 20;
pub const ENEMY_SQUID_POINTS: u64 = 40;

// 1:2 aspect ratio
pub const BULLET_WIDTH: f64 = 10.0;
pub const BULLET_HEIGHT: f64 = BULLET_WIDTH * 2.0;

pub const PLAYER_VELOCITY: Vector2<f64> = Vector2 { x: 128.0, y: 0.0 };
pub const PLAYER_BULLET_ACCELERATION: Vector2<f64> = Vector2 { x: 0.0, y: -15.0 };
pub const PLAYER_BULLET_VELOCITY: Vector2<f64> = Vector2 { x: 0.0, y: -50.0 };

pub const ENEMY_BULLET_ACCELERATION: Vector2<f64> = Vector2 { x: 0.0, y: 0.0 };
pub const ENEMY_BULLET_VELOCITY: Vector2<f64> = Vector2 { x: 0.0, y: 16.0 };

pub const PLAYER_BULLETS_FREQUENCY: u32 = 10;

pub const FONT_NAME: &str = "AmazS.T.A.L.K.E.R.v.3.0.ttf";

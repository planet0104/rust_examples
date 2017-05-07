use std::f64::consts::PI;

pub const FRAMES_PER_SECOND: u32 = 60;
pub const NUM_INPUTS: usize = 1;
pub const NUM_OUTPUTS: usize = 2;
pub const MAX_TURN_RATE: f64 = 0.2;
//pub const MAX_SPEED: f64 = 2.0;
pub const NUM_MINES: usize = 40;
pub const NUM_SWEEPERS: usize = 30;
pub const NUM_TICKS: i32 = 2000;
pub const MINE_SCALE: f64 = 2.0;
pub const START_ENERGY: f64 = 20.0;
pub const ENERGY_COST_PER_TICK: f64 = 0.001;

pub const SWEEPER_SCALE: i32 = 5;

pub const WINDOW_WIDTH:i32 = 405;
pub const WINDOW_HEIGHT:i32 = 405;

pub const TWO_PI:f64 = PI*2.0;
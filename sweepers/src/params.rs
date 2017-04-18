// use std::f64::consts::PI;

// pub const FRAMES_PER_SECOND: u32 = 60;
// //pub const NUM_INPUTS: usize = 4;
// pub const NUM_INPUTS: usize = 1;
// pub const NUM_HIDDEN: usize = 1;
// pub const NEURONS_PER_HIDDEN_LAYER: usize = 10;
// pub const NUM_OUTPUTS: usize = 2;
// pub const ACTIVATION_RESPONSE: f64 = 1.0;
// pub const BIAS: f64 = -1.0;
// pub const MAX_TURN_RATE: f64 = 0.3;
// //pub const MAX_SPEED: f64 = 2.0;
// pub const NUM_MINES: usize = 40;
// pub const NUM_SWEEPERS: usize = 30;
// pub const NUM_TICKS: i32 = 2000;
// pub const MINE_SCALE: f64 = 2.0;
// pub const START_ENERGY: f64 = 20;
// pub const ENERGY_COST_PER_TICK = 0.001;

// pub const CROSSOVER_RATE: f64 = 0.7;
// pub const MUTATION_RATE: f64 = 0.1;
// pub const MAX_PERTURBATION: f64 = 0.3;
// pub const NUM_ELITE: usize = 4;
// pub const NUM_COPIES_ELITE: usize = 1;

// pub const SWEEPER_SCALE: i32 = 5;

// pub const WINDOW_WIDTH:i32 = 405;
// pub const WINDOW_HEIGHT:i32 = 405;

// pub const TWO_PI:f64 = PI*2.0;

use std::f64::consts::PI;

pub const FRAMES_PER_SECOND: u32 = 60;
//pub const NUM_INPUTS: usize = 4;
pub const NUM_INPUTS: usize = 1;
pub const NUM_HIDDEN: usize = 1;
pub const NEURONS_PER_HIDDEN_LAYER: usize = 10;
pub const NUM_OUTPUTS: usize = 2;
pub const ACTIVATION_RESPONSE: f64 = 1.0;
pub const BIAS: f64 = -1.0;
pub const MAX_TURN_RATE: f64 = 0.2;
//pub const MAX_SPEED: f64 = 2.0;
pub const NUM_MINES: usize = 40;
pub const NUM_SWEEPERS: usize = 30;
pub const NUM_TICKS: i32 = 2000;
pub const MINE_SCALE: f64 = 2.0;
pub const START_ENERGY: f64 = 20.0;
pub const ENERGY_COST_PER_TICK: f64 = 0.001;

pub const CROSSOVER_RATE: f64 = 0.7;
pub const MUTATION_RATE: f64 = 0.1;
pub const MAX_PERTURBATION: f64 = 0.3;
pub const NUM_ELITE: usize = 4;
pub const NUM_COPIES_ELITE: usize = 1;

pub const SWEEPER_SCALE: i32 = 5;

pub const WINDOW_WIDTH:i32 = 405;
pub const WINDOW_HEIGHT:i32 = 405;

pub const TWO_PI:f64 = PI*2.0;
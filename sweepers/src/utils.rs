extern crate rand;
use std::time::{Duration, Instant};
use rand::{Rng};

#[derive(Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new() -> Point {
        Point{ x: 0, y: 0}
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point{ x: self.x, y: self.y }
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
    }
}

#[derive(Debug)]
pub struct PointF{
    pub x: f32,
    pub y: f32
}

impl PointF {
    pub fn new() -> PointF {
        PointF{ x: 0.0, y: 0.0}
    }

    pub fn from(x: f32, y: f32) -> PointF {
        PointF{ x: x, y: y}
    }
}

impl Clone for PointF {
    fn clone(&self) -> PointF {
        PointF{ x: self.x, y: self.y }
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
    }
}

pub struct Timer{
    fps: u32,
    frame_time: Duration,
    start_time: Instant,
    next_time: Duration
}

impl Timer{
    pub fn new(fps: u32) ->Timer{
        Timer{
            fps: fps,
            frame_time: Duration::new(1, 0)/fps,
            start_time: Instant::now(),
            next_time: Duration::new(0, 0)
        }
    }

    // pub fn init(&mut self, fps: u32){
    //     self.fps = fps;
    //     self.frame_time = Duration::new(1, 0)/fps;
    // }

    pub fn ready_for_next_frame(&mut self) -> bool{
        assert!(self.fps>0);
        let current_time = self.start_time.elapsed();
        let mut ret = false;
        if current_time > self.next_time {
            self.next_time = current_time + self.frame_time;
            ret = true
        }
        ret
    }
}

//返回-1 <n <1范围内的随机浮点数
pub fn random_clamped() -> f64{ rand::random::<f64>() - rand::random::<f64>() }
pub fn random_float() -> f64{ rand::random::<f64>() }
//返回[low, low] 区间的数
pub fn random_int(low: i32, high: i32) -> i32{
    //返回[low, high)区间的数
    //println!("low={},high={}", low, high);
    rand::thread_rng().gen_range(low, high+1)
}

pub fn clamp(arg: &mut f64, min: f64, max: f64){
    if *arg < min {
        *arg = min;
    }
    if *arg > max {
        *arg = max;
    }
}

pub fn rgb(red: i32, green: i32, blue: i32) ->u32 {
    red as u32 | (green as u32)<<8 | (blue as u32)<<16
}
mod matrix;
mod params;
mod utils;
mod event_manager;
// mod devices;
mod controller;
mod gen_alg;
mod mine_sweeper;
mod neural_net;
mod vector_2d;
use controller::Controller;
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::{Key, Window, WindowOptions};
use params::{FRAMES_PER_SECOND, WINDOW_HEIGHT, WINDOW_WIDTH};
use raqote::{
    DrawTarget, SolidSource,
};
use utils::{Timer};
use event_manager::*;

fn main() {
    let mut window = Window::new(
        "聪明的扫雷机 v1.0",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::Title(String::from("微软雅黑"))], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut controller = Controller::new();
    let mut timer = Timer::new(FRAMES_PER_SECOND);

    let mut event_manager = EventManager::new();
    event_manager.add_key(Key::F);
    event_manager.add_key(Key::N);
    event_manager.add_key(Key::R);

    while window.is_open(){

        event_manager.update(&window);
        if let Some(is_key_down) = event_manager.get_event(&Key::F){
            if !is_key_down {
                controller.fast_render_toggle();
            }
        };

        if let Some(is_key_down) = event_manager.get_event(&Key::N){
            if !is_key_down {
                controller.render_enable_toggle();
            }
        };

        if let Some(is_key_down) = event_manager.get_event(&Key::R){
            if !is_key_down {
                controller = Controller::new();
            }
        };

        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));

        let ready_render = timer.ready_for_next_frame();
        if ready_render || controller.fast_render() {
            //println!("next frame..");
            if !controller.update() {
                //遇到问题，结束程序
                println!("遇到问题，程序结束");
                break;
            }
        }

        if ready_render{
            //绘制
            controller.render(&mut dt, &font);
            window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        }
    }
}

use winit::dpi::LogicalSize;
use winit::{WindowEvent, Event};
use std::thread;
use std::time::{Instant, Duration};
use direct2d::image::Bitmap;
use direct2d::brush::SolidColorBrush;
use dxgi::enums::*;
use direct2d::enums::{DrawTextOptions, PresentOptions, RenderTargetType, BitmapInterpolationMode, RenderTargetUsage};
use winapi::shared::windef::HWND;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use math2d::*;
use directwrite::text_format::TextFormat;

const WINDOW_WIDTH:i32 = 640;
const WINDOW_HEIGHT:i32 = 480;

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64))
        .with_title("Test")
        .build(&events_loop)
        .unwrap();

    // #[cfg(target_os = "windows")]
    // {
    

    // }

    use winit::os::windows::WindowExt;
    let hwnd = window.get_hwnd() as HWND;

    let d2d = direct2d::factory::Factory::new().unwrap();
    use direct2d::render_target::hwnd::HwndRenderTargetBuilder;
    let target = Rc::new(RefCell::new(HwndRenderTargetBuilder::new(&d2d)
    .with_hwnd(hwnd)
    .with_usage(RenderTargetUsage::NONE)
    .with_target_type(RenderTargetType::Default)
    .with_pixel_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
    .with_present_options(PresentOptions::NONE)
    // .with_dpi(20.0, 20.0)
    .build().unwrap()));

    let fg_brush = SolidColorBrush::create(& *target.borrow())
        .with_color(Color::BLACK)
        .build()
        .unwrap();

    let bg_brush = SolidColorBrush::create(& *target.borrow())
        .with_color(Color::RED)
        .build()
        .unwrap();

    let mut ups = 0;
    let mut fps = 0;

    let client_width = Rc::new(RefCell::new(WINDOW_WIDTH));
    let client_height = Rc::new(RefCell::new(WINDOW_HEIGHT));
    let mut exit = false;

    let fpsd = |fps:u64| -> Duration{
        Duration::from_micros(1000*1000/fps)
    };

    let update_timer = Instant::now();
    let mut next_update_time = update_timer.elapsed();
    let mut next_log_time =  update_timer.elapsed();
    let mut next_draw_time =  update_timer.elapsed();

    //加载一张图片
    let decoder = png::Decoder::new(File::open("bird.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();
    println!("info.bit_depth={:?}", info.bit_depth);
    println!("info.buffer_size={:?}", info.buffer_size());
    println!("info.color_type={:?}", info.color_type);
    println!("info.height={:?}", info.height);
    println!("info.line_size={:?}", info.line_size);
    println!("info.width={:?}", info.width);
    
    let bitmap = Bitmap::create(& *target.borrow())
                .with_format(Format::R8G8B8A8Unorm)
                .with_raw_data(Sizeu::new(info.width, info.height), &buf, 34*4)
                .build().unwrap();

    let dw = directwrite::factory::Factory::new().unwrap();

    let context = target.clone();
    let draw = ||{
        let t = Instant::now();
        let mut context = context.borrow_mut();
        context.begin_draw();

        // Make the background clear
        context.clear(Color::WHITE);
        context.fill_rectangle(Rectf::new(0., 0., 100., 100.), &bg_brush);
        context.draw_rectangle(Rectf::new(0., 0., 100., 100.), &fg_brush, 1.0, None);
        let dest_rect = Rectf::new(0.0, 0.0, 34.0, 24.0);
        let src_rect = Rectf::new(0.0, 0.0, 34.0, 24.0);
        let transform = Matrix3x2f::rotation(0.5, [17., 12.])
                        * Matrix3x2f::translation([34.0, 24.0]);
        context.set_transform(&transform);
        context.draw_bitmap(&bitmap, dest_rect, 1.0, BitmapInterpolationMode::Linear, src_rect);
        context.set_transform(&Matrix3x2f::IDENTITY);

        //绘制文字
        let text_format = TextFormat::create(&dw)
        .with_family("")
        .with_size(20.0)
        .build().unwrap();
        context.draw_text("Hello世界!", &text_format, [100.0, 100.0, 200.0, 140.0], &fg_brush, DrawTextOptions::NONE);
        // println!("绘图耗时:{}微妙", t.elapsed().as_micros());

        // Finish
        context.end_draw().unwrap();
        // println!("END绘图耗时:{}微妙", t.elapsed().as_micros());
    };

    while !exit{
        if update_timer.elapsed()>next_update_time{
            next_update_time = next_update_time+fpsd(200);
            ups += 1;
        }
        if update_timer.elapsed()>next_draw_time{
            draw();
            next_draw_time = next_draw_time+fpsd(70);
            fps += 1;
        }
        if update_timer.elapsed()>next_log_time{
            next_log_time = next_log_time+fpsd(1);
            println!("ups={} fps={} {}x{}", ups, fps, client_width.borrow(), client_height.borrow());
            ups = 0;
            fps = 0;
        }
        events_loop.poll_events(|event| {
            // println!("{:?}", event);
            match event {
                Event::WindowEvent{ event: winit::WindowEvent::Resized(size), .. } => {
                    *client_width.borrow_mut() = size.width as i32;
                    *client_height.borrow_mut() = size.height as i32;
                    let _ = target.borrow_mut().resize(Sizeu::new(size.width as u32, size.height as u32));
                    winit::ControlFlow::Continue
                }
                Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } =>{
                    exit = true;
                    winit::ControlFlow::Break
                }
                _ => winit::ControlFlow::Continue,
            };
        });
        // thread::sleep(Duration::from_nanos(1));
    }
}
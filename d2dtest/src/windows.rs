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
use std::sync::{Arc, Mutex};
use directwrite::text_format::TextFormat;
use direct2d::render_target::hwnd::HwndRenderTarget;
use std::sync::mpsc::channel;
use image::{RgbaImage, DynamicImage};
use std::sync::mpsc::Sender;
use std::io::prelude::*;
use std::io::{Result, ErrorKind, Error};

const WINDOW_WIDTH:i32 = 640;
const WINDOW_HEIGHT:i32 = 480;

pub enum AssetsType{
    Image,
    File
}

pub enum Assets{
    Image(Bitmap),
    File(Vec<u8>),
}

pub enum RawAssets{
    Image(RgbaImage),
    File(Vec<u8>),
}

pub trait Window{
    fn load_assets(&mut self, assets:Vec<(String, AssetsType)>);
}

pub trait Graphics{
    fn clear(&mut self, r:u8, g:u8, b:u8);
}

pub trait State{
    fn new(window:&mut Window) -> Self;
    fn update(&mut self, window:&mut Window);
    fn draw(&mut self, graphics:&mut Graphics, window:&mut Window);
    fn on_assets_load(&mut self, path:&str, assets:std::io::Result<Assets>, window:&mut Window);
}

struct D2DGraphics{
    target: HwndRenderTarget
}
impl Graphics for D2DGraphics{
    fn clear(&mut self, r:u8, g:u8, b:u8){
        self.target.clear(Color::new(r as f32/255.0, g as f32/255.0, b as f32/255.0, 1.0));
    }
}

struct D2DWindow{
    thread_sender: Sender<(String, AssetsType, Result<RawAssets>)>
}

impl Window for D2DWindow{
    fn load_assets(&mut self, assets:Vec<(String, AssetsType)>){
        let sender = self.thread_sender.clone();
        //启动线程读取所有文件
        thread::spawn(move || {
            for (path, tp) in assets{
                match File::open(&path){
                    Ok(mut f) => {
                        let mut buf = vec![];
                        match f.read_to_end(&mut buf){
                            Ok(_len) => {
                                match tp{
                                    AssetsType::Image => {
                                        if let Ok(image) = image::load_from_memory(&buf){
                                            let _ = sender.send((path, tp, Ok(RawAssets::Image(image.to_rgba()))));
                                        }else{
                                            let _ = sender.send((path, tp, Err(Error::new(ErrorKind::Other, "图片读取失败"))));
                                        }
                                    }
                                    AssetsType::File => {
                                        //将文件数据发送到主线程    
                                        let _ = sender.send((path, tp, Ok(RawAssets::File(buf))));
                                    }
                                }
                            },
                            Err(err) => {
                                let _ = sender.send((path, tp, Err(err)));
                            }
                        }
                    },
                    Err(err) => {
                        let _ = sender.send((path, tp, Err(err)));
                    }
                }
            }
        });
    }
}

pub fn run<S: State>() {

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
    let target = HwndRenderTargetBuilder::new(&d2d)
    .with_hwnd(hwnd)
    .with_usage(RenderTargetUsage::NONE)
    .with_target_type(RenderTargetType::Default)
    .with_format(Format::R8G8B8A8Unorm)
    .with_alpha_mode(direct2d::enums::AlphaMode::Ignore)
    .with_pixel_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
    .with_present_options(PresentOptions::NONE)
    // .with_dpi(20.0, 20.0)
    .build().unwrap();

    let graphics = Arc::new(Mutex::new(D2DGraphics{target}));

    // let bg_brush = SolidColorBrush::create(&graphics.target)
    //     .with_color(Color::RED)
    //     .build()
    //     .unwrap();

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

    //加载一张图片
    // let decoder = png::Decoder::new(File::open("bird.png").unwrap());
    // let (info, mut reader) = decoder.read_info().unwrap();
    // let mut buf = vec![0; info.buffer_size()];
    // reader.next_frame(&mut buf).unwrap();
    // println!("info.bit_depth={:?}", info.bit_depth);
    // println!("info.buffer_size={:?}", info.buffer_size());
    // println!("info.color_type={:?}", info.color_type);
    // println!("info.height={:?}", info.height);
    // println!("info.line_size={:?}", info.line_size);
    // println!("info.width={:?}", info.width);
    
    

    let dw = directwrite::factory::Factory::new().unwrap();

    // let context = target.clone();
    // let draw = ||{
    //     let t = Instant::now();
    //     let mut context = context.borrow_mut();
    //     context.begin_draw();

    //     // Make the background clear
    //     context.clear(Color::WHITE);
    //     context.fill_rectangle(Rectf::new(0., 0., 100., 100.), &bg_brush);
    //     context.draw_rectangle(Rectf::new(0., 0., 100., 100.), &fg_brush, 1.0, None);
    //     let dest_rect = Rectf::new(0.0, 0.0, 34.0, 24.0);
    //     let src_rect = Rectf::new(0.0, 0.0, 34.0, 24.0);
    //     let transform = Matrix3x2f::rotation(0.5, [17., 12.])
    //                     * Matrix3x2f::translation([34.0, 24.0]);
    //     context.set_transform(&transform);
    //     context.draw_bitmap(&bitmap, dest_rect, 1.0, BitmapInterpolationMode::Linear, src_rect);
    //     context.set_transform(&Matrix3x2f::IDENTITY);

    //     //绘制文字
    //     let text_format = TextFormat::create(&dw)
    //     .with_family("")
    //     .with_size(20.0)
    //     .build().unwrap();
    //     context.draw_text("Hello世界!", &text_format, [100.0, 100.0, 200.0, 140.0], &fg_brush, DrawTextOptions::NONE);
    //     // println!("绘图耗时:{}微妙", t.elapsed().as_micros());

    //     // Finish
    //     context.end_draw().unwrap();
    //     // println!("END绘图耗时:{}微妙", t.elapsed().as_micros());
    // };

    // let drawing = Arc::new(Mutex::new(()));

    let (thread_sender, main_receiver) = channel();
    let (main_sender, thread_receiver) = channel();
    let (assets_sender, assets_receiver) = channel();

    let g = graphics.clone();
    thread::spawn(move || {
        //接收到draw消息，调用begin_draw，然后等待end_draw消息
        let begin_draw = ||{
            g.lock().unwrap().target.begin_draw();
        };

        loop{
            begin_draw();
            let _ = thread_sender.send("begin");
            if let Ok(msg) = thread_receiver.recv(){
                if msg == "draw"{
                    let _ = g.lock().unwrap().target.end_draw();
                }
            }
        }
    });

    let mut raw_rgba_images = vec![];
    let mut game_window = D2DWindow{thread_sender: assets_sender};
    let mut state = S::new(&mut  game_window);

    while !exit{
        if update_timer.elapsed()>next_update_time{
            next_update_time = next_update_time+fpsd(60);
            state.update(&mut game_window);
            ups += 1;
        }
        if let Ok((path, tp, data)) = assets_receiver.try_recv(){
            match data{
                Ok(RawAssets::File(data)) => {
                    state.on_assets_load(&path, Ok(Assets::File(data)), &mut game_window)
                }
                Ok(RawAssets::Image(image)) => {
                    raw_rgba_images.push((path, image));
                }
                Err(err) => state.on_assets_load(&path, Err(err), &mut game_window)
            };
        }

        if let Ok(_msg) = main_receiver.try_recv(){
            let mut g = graphics.lock().unwrap();
            //加载图片资源
            if let Some((path, data)) = raw_rgba_images.pop(){
                let (w, h) = (data.width(), data.height());
                let buf = data.into_raw();
                match Bitmap::create(&g.target)
                .with_format(Format::R8G8B8A8Unorm)
                .with_raw_data(Sizeu::new(w, h), &buf, w*4)
                .build(){
                    Ok(bmp) => state.on_assets_load(&path, Ok(Assets::Image(bmp)), &mut game_window),
                    Err(err) => state.on_assets_load(&path, Err(Error::new(ErrorKind::Other, format!("{:?}", err))), &mut game_window),
                };
            }
            //绘制文字
            let fg_brush = SolidColorBrush::create(&g.target)
            .with_color(Color::BLACK)
            .build()
            .unwrap();

            let text_format = TextFormat::create(&dw)
            .with_family("")
            .with_size(20.0)
            .build().unwrap();
            // println!("绘图耗时:{}微妙", t.elapsed().as_micros());
            state.draw(&mut  *g, &mut game_window);
            g.target.draw_text("你好世界！hello!", &text_format, [0.0, 0.0, 20.0*11.0, 20.0], &fg_brush, DrawTextOptions::NONE);
            let _ = main_sender.send("draw");
        }
        // if update_timer.elapsed()>next_draw_time{
        //     // state.draw();//try_lock 
        //     // draw();
            
        //     next_draw_time = next_draw_time+fpsd(60);
        //     fps += 1;
        // }
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
                    // let _ = target.borrow_mut().resize(Sizeu::new(size.width as u32, size.height as u32));
                    winit::ControlFlow::Continue
                }
                Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } =>{
                    exit = true;
                    winit::ControlFlow::Break
                }
                _ => winit::ControlFlow::Continue,
            };
        });
        thread::sleep(Duration::from_nanos(1));
    }
}
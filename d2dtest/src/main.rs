use winit::dpi::LogicalSize;
use winit::{WindowEvent, Event};
use std::thread;
use std::time::{Instant, Duration};
use com_wrapper::ComWrapper;
use direct2d::{DeviceContext, RenderTarget};
use direct2d::brush::SolidColorBrush;
use direct2d::image::Bitmap;
use direct2d::device::Device;
use direct2d::render_target::hwnd::HwndRenderTargetBuilder;
use direct2d::descriptions::PixelFormat;
use direct2d::image::bitmap1::Bitmap1;
use direct2d::enums::{BitmapInterpolationMode, BitmapOptions};
use winapi::shared::windef::HWND;
use direct3d11::enums::{BindFlags, CpuAccessFlags, CreateDeviceFlags, Usage};
use dxgi::enums::{Format, MapFlags};
use math2d::{Matrix3x2f, Point2f};
use math2d::color::Color;
use winapi::um::winuser::{ReleaseDC, GetDC};
use winapi::um::wingdi::{StretchDIBits, DIB_RGB_COLORS, BITMAPINFOHEADER, SRCCOPY, BITMAPINFO, BI_RGB};
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::shared::basetsd::UINT_PTR;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use math2d::Sizeu;

const WINDOW_WIDTH:i32 = 640;
const WINDOW_HEIGHT:i32 = 480;

thread_local!{
    static COUNT:RefCell<(u64, Timer)> = RefCell::new((0, Timer::new(1)));
}

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

    let d2d = direct2d::factory::Factory1::new().unwrap();
    // Initialize a D3D Device
    let (_, d3d, d3d_ctx) = direct3d11::device::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();

    // Create the D2D Device and Context
    let device = Device::create(&d2d, &d3d.as_dxgi()).unwrap();
    let mut context = DeviceContext::create(&device).unwrap();

    // Create a texture to render to
    let tex = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .with_format(Format::R8G8B8A8Unorm)
        .with_bind_flags(BindFlags::RENDER_TARGET | BindFlags::SHADER_RESOURCE)
        .build()
        .unwrap();

    // Bind the backing texture to a D2D Bitmap
    let mut target = Bitmap1::create(&context)
        .with_dxgi_surface(&tex.as_dxgi())
        // .with_dpi(DPI, DPI)
        .with_options(BitmapOptions::TARGET)
        .build()
        .unwrap();

    context.set_target(&mut target);
    // context.set_dpi(DPI, DPI);

    let fg_brush = SolidColorBrush::create(&context)
        .with_color(Color::new(0.0, 0.0, 0.0, 1.0))
        .build()
        .unwrap();

    let bg_brush = SolidColorBrush::create(&context)
        .with_color(Color::new(1.0, 0.0, 0.0, 1.0))
        .build()
        .unwrap();

    let mut bitmap_info:BITMAPINFO = unsafe{ std::mem::zeroed() };
    let size = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bitmap_info.bmiHeader.biBitCount = 24;
    bitmap_info.bmiHeader.biWidth = WINDOW_WIDTH;
    bitmap_info.bmiHeader.biHeight = WINDOW_HEIGHT;
    bitmap_info.bmiHeader.biPlanes = 1;
    bitmap_info.bmiHeader.biSize = size;
    // bitmap_info.bmiHeader.biSizeImage = 0;
    bitmap_info.bmiHeader.biCompression = BI_RGB;

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
    
    // use direct2d::image::bitmap::builder::BitmapBuilder;
    // use direct2d::image::bitmap1::builder::BitmapBuilder1;
    let bitmap = Bitmap1::create(&context)
                .with_format(Format::R8G8B8A8Unorm)
                .with_image_data(Sizeu::new(info.width, info.height), &buf, 34*4)
                .build().unwrap();

    let (cw, ch) = (client_width.clone(), client_height.clone());
    let mut draw = ||{
        let t = Instant::now();

        context.begin_draw();

        // Make the background clear
        context.clear(Color::new(1.0, 1.0, 1.0, 1.0));
        let rect = [0.0, 0.0, 100.0, 100.0];
        context.fill_rectangle(rect, &bg_brush);
        context.draw_rectangle(rect, &fg_brush, 1.0, None);
        context.draw_bitmap(&bitmap, [200.0, 200.0, 234.0, 224.0], 1.0, BitmapInterpolationMode::Linear, [0.0, 0.0, 34.0, 24.0]);
        
        // Draw the hexagon
        // let transform = Matrix3x2f::scaling(99.0 / 100.0, Point2f::ORIGIN)
        //             * Matrix3x2f::translation([50.0, 50.0]);
        // context.set_transform(&transform);
        // context.fill_geometry(&hex, &bg_brush);
        // context.draw_geometry(&hex, &fg_brush, 1.0, None)

        // Finish
        context.end_draw().unwrap();
        // println!("绘图耗时:{}微妙", t.elapsed().as_micros());let t = Instant::now();

        let temp_texture = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .with_format(Format::R8G8B8A8Uint)
        .with_bind_flags(BindFlags::NONE)
        .with_usage(Usage::Staging)
        .with_cpu_access(CpuAccessFlags::READ)
        .build()
        .unwrap();

        // println!("创建texture耗时:{}微妙", t.elapsed().as_micros());let t = Instant::now();

        // Get the data so we can write it to a file
        // TODO: Have a safe way to accomplish this :D
        let mut raw_pixels: Vec<u8> = Vec::with_capacity(WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize * 4);
        unsafe {
            let ctx = &*d3d_ctx.get_raw();
            ctx.CopyResource(temp_texture.get_raw() as *mut _, tex.get_raw() as *mut _);
            ctx.Flush();

            let surface = temp_texture.as_dxgi();
            let map = surface.map(MapFlags::READ).unwrap();
            // println!("创建raw_pixels耗时0:{}微妙", t.elapsed().as_micros());let t = Instant::now();
            for y in (0..WINDOW_HEIGHT).rev() {
                for pixel in map.row(y as u32)[..WINDOW_WIDTH as usize * 4].chunks(4){
                    raw_pixels.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
                }
            }
            // println!("创建raw_pixels耗时1:{}微妙", t.elapsed().as_micros());let t = Instant::now();
        }

        // image::save_buffer(
        //         "temp-image.png",
        //         &raw_pixels,
        //         WINDOW_WIDTH as u32,
        //         WINDOW_HEIGHT as u32,
        //         image::ColorType::RGBA(8)).unwrap();
        
        unsafe{
            let dc = GetDC(hwnd);
            StretchDIBits(dc, 0, 0, *cw.borrow(), *ch.borrow(), 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, raw_pixels.as_ptr() as *const std::ffi::c_void, &bitmap_info, DIB_RGB_COLORS, SRCCOPY);
            ReleaseDC(hwnd, dc);
        }
        println!("StretchDIBits耗时:{}微妙", t.elapsed().as_micros());
    };

    while !exit{
        if update_timer.elapsed()>next_update_time{
            next_update_time = next_update_time+fpsd(70);
            ups += 1;
        }
        if update_timer.elapsed()>next_draw_time{
            draw();
            next_draw_time = next_draw_time+fpsd(1);
            fps += 1;
        }
        if update_timer.elapsed()>next_log_time{
            next_log_time = next_log_time+fpsd(1);
            println!("ups={} fps={}", ups, fps);
            ups = 0;
            fps = 0;
        }
        events_loop.poll_events(|event| {
            // println!("{:?}", event);
            match event {
                Event::WindowEvent{ event: winit::WindowEvent::Resized(size), .. } => {
                    *client_width.borrow_mut() = size.width as i32;
                    *client_height.borrow_mut() = size.height as i32;
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

pub fn current_timestamp() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as f64
}

//计时器
// #[derive(Clone)]
// pub struct AnimationTimer {
//     frame_time: f64,
//     next_time: f64,
// }

// impl AnimationTimer {
//     pub fn new(fps: f64) -> AnimationTimer {
//         AnimationTimer {
//             frame_time: 1000.0 / fps,
//             next_time: current_timestamp(),
//         }
//     }

//     pub fn set_fps(&mut self, fps: f64){
//         self.frame_time = 1000.0 / fps;
//     }

//     pub fn reset(&mut self) {
//         self.next_time = current_timestamp();
//     }

//     pub fn ready_for_next_frame(&mut self) -> bool {
//         let now = current_timestamp();
//         if now >= self.next_time {
//             //更新时间
//             self.next_time += self.frame_time;
//             true
//         } else {
//             false
//         }
//     }
// }

pub struct Timer {
    fps: u32,
    frame_time: Duration,
    start_time: Instant,
    next_time: Duration,
}

impl Timer {
    pub fn new(fps: u32) -> Timer {
        Timer {
            fps: fps,
            frame_time: Duration::new(1, 0) / fps,
            start_time: Instant::now(),
            next_time: Duration::new(0, 0),
        }
    }

    // pub fn init(&mut self, fps: u32){
    //     self.fps = fps;
    //     self.frame_time = Duration::new(1, 0)/fps;
    // }

    pub fn ready_for_next_frame(&mut self) -> bool {
        assert!(self.fps > 0);
        let current_time = self.start_time.elapsed();
        let mut ret = false;
        if current_time > self.next_time {
            self.next_time = current_time + self.frame_time;
            ret = true
        }
        ret
    }
}
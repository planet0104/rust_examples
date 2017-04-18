#![no_main]
mod params;
mod matrix;
mod utils;
mod devices;
mod neural_net;
mod gen_alg;
mod mine_sweeper;
mod vector_2d;
mod controller;
extern crate winapi;
extern crate user32;
extern crate gdi32;
extern crate rand;
use winapi::windef::{HDC, HWND, HBITMAP, HGDIOBJ};
use winapi::minwindef::{HIWORD, LOWORD, UINT, WPARAM, LPARAM, LRESULT };
use winapi::winuser::{ VK_SPACE, WM_SIZE, WM_KEYUP, WM_CREATE, WM_DESTROY, PAINTSTRUCT, WM_PAINT};
use winapi::wingdi::{ WHITENESS,SRCCOPY };
use params::{ FRAMES_PER_SECOND, WINDOW_WIDTH,WINDOW_HEIGHT };
use utils::{ Point,Timer };
use devices::{ Window, new_rect, new_paint_st};
use std::mem::transmute;
use std::ptr;
use controller::{ Controller };

const APP_NAME: &'static str = "聪明的扫雷机 v1.0";
//控制器
static mut CONTROLLER:*const Controller = 0 as *const Controller;
//Timer
static mut TIMER:*mut Timer = 0 as *mut Timer;

static mut HDC_BACK_BUFFER: HDC = 0 as HDC;//后置缓冲区
static mut H_BITMAP : HBITMAP = 0 as HBITMAP;
static mut H_OLD_BITMAP : HBITMAP = 0 as HBITMAP;
static mut CX_CLIENT : i32 = WINDOW_WIDTH;
static mut CY_CLIENT : i32 = WINDOW_HEIGHT;

//窗口消息函数
pub unsafe extern "system" fn window_proc(h_wnd :HWND, u_msg :UINT, w_param :WPARAM, l_param :LPARAM) -> LRESULT {
    //WM_KEYUP 按空格键退出
    if u_msg == WM_CREATE {
        let mut rect: winapi::RECT = new_rect();
        user32::GetClientRect(h_wnd, &mut rect);
        CX_CLIENT = rect.right;
        CY_CLIENT = rect.bottom;

        //初始化 controller
        controller();

        //创建绘图缓冲区(HDC)
        HDC_BACK_BUFFER = gdi32::CreateCompatibleDC(0 as HDC);
        let hdc:HDC = user32::GetDC(h_wnd);
        
        //创建绘图缓冲区的Bitmap
        H_BITMAP = gdi32::CreateCompatibleBitmap(hdc, CX_CLIENT, CY_CLIENT);
        user32::ReleaseDC(h_wnd, hdc);
        //将Bitmap选入缓冲区
        H_OLD_BITMAP = gdi32::SelectObject(HDC_BACK_BUFFER, H_BITMAP as HGDIOBJ) as HBITMAP;

    }else if u_msg == WM_PAINT{

        let mut paint_rect: PAINTSTRUCT = new_paint_st();
        user32::BeginPaint(h_wnd, &mut paint_rect);

        //绘制地雷和扫雷机
        controller().render(paint_rect.hdc);
        
        user32::EndPaint(h_wnd, &mut paint_rect);
    }else if u_msg == WM_DESTROY {
        println!(">>WM_DESTROY...");
        gdi32::SelectObject(HDC_BACK_BUFFER, H_OLD_BITMAP as HGDIOBJ);

        //清理后置缓冲区对象
        gdi32::DeleteDC(HDC_BACK_BUFFER);
        gdi32::DeleteObject(H_BITMAP as HGDIOBJ);

        //杀死程序
        user32::PostQuitMessage(0);
    }else if u_msg == WM_KEYUP {
        if w_param == VK_SPACE as u64 {
            //user32::PostQuitMessage(0);
        }else if w_param == 'F' as u64 {
            controller().fast_render_toggle();
        }else if w_param == 'N' as u64 {
            controller().render_enable_toggle();
        }else if w_param == 'R' as u64 {
            //销毁之前的controller
            drop(controller());
            //重新创建Controller
            controller();
        }
    //有用户调整客户区的大小吗？
    }else if u_msg == WM_SIZE {
        CX_CLIENT = LOWORD(l_param as u32) as i32;
        CY_CLIENT = HIWORD(l_param as u32) as i32;
        //相应地调整backbuffer的大小
        gdi32::SelectObject(HDC_BACK_BUFFER, H_OLD_BITMAP as HGDIOBJ);

        let hdc:HDC = user32::GetDC(h_wnd);

        H_BITMAP = gdi32::CreateCompatibleBitmap(hdc, CX_CLIENT, CY_CLIENT);

        user32::ReleaseDC(h_wnd, hdc);

        H_OLD_BITMAP = gdi32::SelectObject(HDC_BACK_BUFFER, H_BITMAP as HGDIOBJ) as HBITMAP;
    }
    return user32::DefWindowProcW(h_wnd, u_msg, w_param, l_param);
}

unsafe fn game_loop(window: &mut Window){
    //println!("game_loop..");
    if timer().ready_for_next_frame() || controller().fast_render() {
        //println!("next frame..");
        if !controller().update() {
            //遇到问题，结束程序
            println!("遇到问题，程序结束");
            window.close();
        }
        //缓冲区填充白色
        let hdc:HDC = user32::GetDC(window.get_handle());
        gdi32::BitBlt(HDC_BACK_BUFFER,
            0,
            0,
            CX_CLIENT,
            CY_CLIENT,
            0 as HDC,
            0,
            0,
            WHITENESS);
        
        //绘制地雷和扫雷机
        controller().render(HDC_BACK_BUFFER);

        //拷贝缓冲区
        gdi32::BitBlt(hdc,
            0,
            0,
            CX_CLIENT,
            CY_CLIENT,
            HDC_BACK_BUFFER,
            0,
            0,
            SRCCOPY);
        user32::ReleaseDC(window.get_handle(), hdc);
    }
}

#[no_mangle] //禁止Rust编译器修改函数名称
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    //创建窗口
    let mut window = Window::new(String::from(APP_NAME),
                WINDOW_WIDTH, WINDOW_HEIGHT,
                Point{x:WINDOW_WIDTH/2, y:WINDOW_HEIGHT/2},
                None,
                window_proc);
    //显示窗口
    window.show();
    //开始消息循环(阻塞)
    window.start_game_loop(game_loop);

    //回收内存
    unsafe {
        ptr::read::<Controller>(CONTROLLER);
        ptr::read::<Timer>(TIMER);
    }
    println!("程序结束, Controller和Timer已销毁.");
    0
}

fn main() {
    //创建窗口
    let mut window = Window::new(String::from(APP_NAME),
                WINDOW_WIDTH, WINDOW_HEIGHT,
                Point{x:WINDOW_WIDTH/2, y:WINDOW_HEIGHT/2},
                None,
                window_proc);
    //显示窗口
    window.show();
    //开始消息循环(阻塞)
    window.start_game_loop(game_loop);

    //回收内存
    unsafe {
        ptr::read::<Controller>(CONTROLLER);
        ptr::read::<Timer>(TIMER);
    }
    println!("程序结束, Controller和Timer已销毁.");
}

fn controller<'a>() -> &'a mut Controller {
    unsafe {
        if CONTROLLER == ptr::null_mut::<Controller>() {
            CONTROLLER = transmute(Box::new(Controller::new()));
        }
        transmute(CONTROLLER)
    }
}

fn timer<'a>() -> &'a mut Timer {
    unsafe {
        if TIMER == ptr::null_mut::<Timer>() {
            TIMER = transmute(Box::new(Timer::new(FRAMES_PER_SECOND)));
        }
        transmute(TIMER)
    }
}
extern crate gdi32;
extern crate user32;
use winapi::{ MB_OK, RECT, SW_SHOW};
use winapi::winnt::{ LPCWSTR, HANDLE };
use winapi::minwindef::{LOWORD,HIWORD, UINT, DWORD, WPARAM, LPARAM, LRESULT, HINSTANCE };
use winapi::windef::{HICON, HGDIOBJ, HPEN, HWND, HDC, HMENU, HBRUSH, POINT, LPPOINT };
use winapi::winuser::{KEYEVENTF_KEYUP, WS_EX_TRANSPARENT, WM_CREATE, WM_PAINT, WM_QUIT, WM_DESTROY, WM_KEYUP, WM_SIZE, PM_REMOVE, PAINTSTRUCT, WS_CAPTION, WS_OVERLAPPED, WS_VISIBLE, IMAGE_ICON, IDI_APPLICATION, MSG, WS_SYSMENU, WNDCLASSW };
use winapi::wingdi::{PS_SOLID, HOLLOW_BRUSH, BLACK_PEN, WHITE_PEN};
use std::ptr::null_mut;
//use winapi::winuser::{ VK_SPACE, WM_SIZE, WM_KEYUP, WM_CREATE, WM_DESTROY, PAINTSTRUCT, WM_PAINT};

pub type Surface = HDC;
pub type Pen = HPEN;
pub type Brush = HBRUSH;
pub const KEY_CODE_SPACE:i32 = 0x20;
pub const KEY_CODE_LEFT:i32 = 0x25;
pub const KEY_CODE_RIGHT:i32 = 0x27;
pub const KEY_CODE_DOWN:i32 = 0x28;
pub const KEY_CODE_UP:i32 = 0x26;
pub const MM_ANISOTROPIC:i32 = 8;

pub fn solid_pen(width:i32, rgb:u32) -> Pen{
    unsafe{
        gdi32::CreatePen(PS_SOLID, width, rgb)
    }
}

pub fn solid_brush(rgb:u32) ->Brush{
    unsafe{
        gdi32::CreateSolidBrush(rgb)
    }
}

pub fn select_pen(h_dc: HDC, pen:Pen)->Pen{
    unsafe{
        gdi32::SelectObject(h_dc, pen as HGDIOBJ) as Pen
    }
}

pub fn select_brush(h_dc: HDC, brush:Brush)->Brush{
    unsafe{
        gdi32::SelectObject(h_dc, brush as HGDIOBJ) as Brush
    }
}

pub fn select_hollow_brush(h_dc: HDC)->Brush{
    unsafe{
        select_brush(h_dc, gdi32::GetStockObject(HOLLOW_BRUSH) as Brush)
    }
}

pub fn select_black_pen(h_dc:HDC) ->Pen{
    unsafe{
        select_pen(h_dc, gdi32::GetStockObject(BLACK_PEN) as Pen)
    }
}

pub fn select_white_pen(h_dc:HDC) ->Pen{
    unsafe{
        select_pen(h_dc, gdi32::GetStockObject(WHITE_PEN) as Pen)
    }
}

pub fn move_to_ex(h_dc: HDC, x: f64, y:f64){
    unsafe{
        gdi32::MoveToEx(h_dc, x as i32, y as i32, 0 as LPPOINT);
    }
}

pub fn move_to_ex_i32(h_dc: HDC, x: i32, y:i32){
    unsafe{
        gdi32::MoveToEx(h_dc, x, y, 0 as LPPOINT);
    }
}

pub fn delete_pen(pen:Pen){
    unsafe{
        gdi32::DeleteObject(pen as HGDIOBJ);
    }
}

pub fn delete_brush(b:Brush){
    unsafe{
        gdi32::DeleteObject(b as HGDIOBJ);
    }
}

pub fn line_to(h_dc: HDC, x: f64, y:f64){
    unsafe{
        gdi32::LineTo(h_dc, x as i32, y as i32);
    }
}

pub fn line_to_i32(h_dc: HDC, x: i32, y:i32){
    unsafe{
        gdi32::LineTo(h_dc, x, y);
    }
}

pub fn text_out(h_dc: Surface, x: i32, y:i32, string: &str){
    unsafe{
        let ws = str_to_ws(&string);
        let len = ws.len()-1;
        gdi32::TextOutW(h_dc, x as i32, y as i32, ws.as_ptr(), len as i32);
    }
}

pub fn message_box(title:&str, msg: &str){
    unsafe{
        user32::MessageBoxW(null_mut(), str_to_ws(&title).as_ptr(), str_to_ws(&title).as_ptr(), MB_OK);
    }
}

/** 画指定背景色的正方形 */
pub fn rectangle(h_dc: Surface, left:i32, top:i32, right:i32, bottom:i32, rgb:u32){
    unsafe{
        //选择一个画笔画
        let pie_brush = gdi32::CreateSolidBrush(rgb);
        let old_brush = gdi32::SelectObject(h_dc, pie_brush as HGDIOBJ);
        gdi32::Rectangle(h_dc, left, top, right, bottom);
        gdi32::SelectObject(h_dc, old_brush as HGDIOBJ);
        gdi32::DeleteObject(pie_brush as HGDIOBJ);
    }
}

pub fn ellipse(h_dc: Surface, left:i32, top:i32, right:i32, bottom:i32){
    unsafe{
        gdi32::Ellipse(h_dc, left, top, right, bottom);
    }
}

pub fn set_pixel(h_dc:Surface, x:i32, y:i32, rgb:u32){
    unsafe{
        gdi32::SetPixel(h_dc, x, y, rgb);
    }
}

pub fn set_text_color(h_dc:Surface, rgb:u32){
    unsafe{
        gdi32::SetTextColor(h_dc, rgb);
    }
}

pub fn set_map_mode(h_dc: Surface, mode: i32){
    unsafe{
        gdi32::SetMapMode(h_dc, mode);
    }
}
pub fn set_viewport_ext_ex(h_dc: Surface, x:i32, y:i32)->bool{
    unsafe{
        gdi32::SetViewportExtEx(h_dc, x, y, null_mut())>0
    }
}
pub fn set_window_ext_ex(h_dc: Surface, x:i32, y:i32)->bool{
    unsafe{
        gdi32::SetWindowExtEx(h_dc, x, y, null_mut())>0
    }
}
pub fn set_viewport_org_ex(h_dc: Surface, x:i32, y:i32)->bool{
    unsafe{
        gdi32::SetViewportOrgEx(h_dc, x, y, null_mut())>0
    }
}

pub fn set_bk_mode_transparent(h_dc:Surface){
    unsafe{
        gdi32::SetBkMode(h_dc, WS_EX_TRANSPARENT as i32);
    }
}

/** 字符串转换成双字 0结尾的数组 */
pub fn str_to_ws(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

pub fn key_down(vk_code: i32) -> bool{
    unsafe{
        user32::GetAsyncKeyState(vk_code) as i32 & 0x8000 != 0
    }
}

pub fn key_down_event(vk_code: u8){
    unsafe{
        user32::keybd_event(vk_code, 0, 0, 0);
    }
}

pub fn key_up_event(vk_code: u8){
    unsafe{
        user32::keybd_event(vk_code, 0, KEYEVENTF_KEYUP, 0);
    }
}

//窗口
pub struct Window{
    position: Point,
    title: String, //窗口名称
    width: i32,
    height: i32,
    wnd_class: WNDCLASSW,
    wnd_class_name: Vec<u16>,
    h_window: HWND,
    st_msg: MSG,
    done: bool
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            user32::UnregisterClassW(self.wnd_class_name.as_ptr(), self.wnd_class.hInstance);
        }
    }
}

impl Window {
    pub fn new(title: String, width: i32, height: i32, position: Point,
        icon_file_name: Option<String>,
        window_proc: unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT) -> Window{

        let wnd_class_name = str_to_ws("super_window");
        
        //创建并窗口类
        let wnd_class = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(window_proc), 
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: 0 as HINSTANCE,
                hIcon: match icon_file_name.as_ref() {
                    Some(file_name) => unsafe{ load_image(file_name.as_str(), IMAGE_ICON) as HICON },
                    _ => unsafe{ user32::LoadIconW(0 as HINSTANCE, IDI_APPLICATION) }
                },
                hCursor: unsafe{ user32::LoadCursorW(0 as HINSTANCE, IDI_APPLICATION) },
                hbrBackground: 16 as HBRUSH,
                lpszMenuName: 0 as LPCWSTR,
                lpszClassName: wnd_class_name.as_ptr(),
        };
        
        Window{
            position: position,
            title: title,
            width: width,
            height: height,
            wnd_class: wnd_class,
            wnd_class_name: wnd_class_name,
            h_window: 0 as HWND,
            st_msg:MSG {
                hwnd : 0 as HWND,
                message : 0 as UINT,
                wParam : 0 as WPARAM,
                lParam : 0 as LPARAM,
                time : 0 as DWORD,
                pt : POINT { x: 0, y: 0, },
            },
            done: false
        }
    }

    pub fn show(&mut self) -> i32 {
        unsafe{
            //注册窗口类
            user32::RegisterClassW(&self.wnd_class);
            //创建窗口
            self.h_window = user32::CreateWindowExW(
                            0,
                            self.wnd_class_name.as_ptr(),
                            str_to_ws(self.title.as_str()).as_ptr(),
                            WS_OVERLAPPED | WS_VISIBLE | WS_CAPTION | WS_SYSMENU,
                            self.position.x, self.position.y,
                            self.width, self.height,
                            0 as HWND, 0 as HMENU, 0 as HINSTANCE, null_mut());
            //显示窗口
            user32::ShowWindow(self.h_window, SW_SHOW);
            user32::UpdateWindow(self.h_window);
        }
        0
    }

    pub fn start_loop(&mut self){
        // 消息循环
        unsafe {
            loop {
                let pm = user32::GetMessageW(&mut self.st_msg, 0 as HWND, 0, 0);
                if pm == 0 {
                    println!("消息循环结束!");
                    break;
                }
                user32::TranslateMessage(&mut self.st_msg);
                user32::DispatchMessageW(&mut self.st_msg);
            }
        }
    }

    pub fn start_game_loop(&mut self, game_loop: unsafe fn(&mut Window)){
        // 消息循环
        while !self.done {
            unsafe {
                while user32::PeekMessageW(&mut self.st_msg, 0 as HWND, 0, 0, PM_REMOVE) == 1 {
                    if self.st_msg.message == WM_QUIT {
                        self.done = true;
                        println!("消息循环结束!");
                    }else {
                        user32::TranslateMessage(&mut self.st_msg);
                        user32::DispatchMessageW(&mut self.st_msg);
                    }
                }
                game_loop(self);
            }
        }
    }

    pub fn close(&mut self){
        self.done = true;
    }

    pub fn get_handle(&self) -> HWND {
        self.h_window
    }
}

unsafe fn load_image(file_name : &str, file_type: UINT) -> HANDLE{
    user32::LoadImageW(0 as HINSTANCE,
                                    str_to_ws(file_name).as_ptr(),//图片文件名
                                        file_type, //IMAGE_BITMAP: UINT = 0; 装载位图 IMAGE_ICON: UINT = 1; 装载位图
                                        //IMAGE_CURSOR = 2 光标
                                        0, 0, //资源宽高自动获取
                                        0x00000010) // as LR_LOADFROMFILE
}

pub fn new_rect()->RECT{ RECT { left: 0, top: 0, right: 0, bottom: 0} }

pub fn new_paint_st()-> PAINTSTRUCT{ PAINTSTRUCT {
            hdc: 0 as HDC,
            fErase: 0,
            rcPaint: new_rect(),
            fRestore: 0,
            fIncUpdate: 0,
            rgbReserved: [0; 32] }
}

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

pub fn rgb(red: i32, green: i32, blue: i32) ->u32 {
    red as u32 | (green as u32)<<8 | (blue as u32)<<16
}
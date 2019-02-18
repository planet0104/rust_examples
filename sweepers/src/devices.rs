use winapi::um::winnt::{ LPCWSTR, HANDLE };
use winapi::shared::minwindef::{UINT, DWORD, WPARAM, LPARAM, LRESULT, HINSTANCE };
use winapi::shared::windef::{RECT, HICON, HWND, HDC, HMENU, HBRUSH, POINT, LPPOINT };
use winapi::um::winuser::{SW_SHOW, WM_QUIT, PM_REMOVE, PAINTSTRUCT, WS_CAPTION, WS_OVERLAPPED, WS_VISIBLE, IMAGE_ICON, IDI_APPLICATION, MSG, WS_SYSMENU, WNDCLASSW };
use std::ptr::null_mut;
use crate::utils::{ Point};

use winapi::um::wingdi;
use winapi::um::winuser;

pub unsafe fn move_to_ex(h_dc: HDC, x: f64, y:f64){
    wingdi::MoveToEx(h_dc, x as i32, y as i32, 0 as LPPOINT);
}

pub unsafe fn line_to(h_dc: HDC, x: f64, y:f64){
    wingdi::LineTo(h_dc, x as i32, y as i32);
}

pub unsafe fn text_out(h_dc: HDC, x: i32, y:i32, string: &str){
    let ws = str_to_ws(&string);
    let len = ws.len()-1;
    wingdi::TextOutW(h_dc, x as i32, y as i32, ws.as_ptr(), len as i32);
}

/** 字符串转换成双字 0结尾的数组 */
pub fn str_to_ws(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

pub unsafe fn key_down(vk_code: i32) -> bool{
    if winuser::GetAsyncKeyState(vk_code) as i32 & 0x8000 != 0{
        true
    }else{
        false   
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
            winuser::UnregisterClassW(self.wnd_class_name.as_ptr(), self.wnd_class.hInstance);
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
                    _ => unsafe{ winuser::LoadIconW(0 as HINSTANCE, IDI_APPLICATION) }
                },
                hCursor: unsafe{ winuser::LoadCursorW(0 as HINSTANCE, IDI_APPLICATION) },
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
            winuser::RegisterClassW(&self.wnd_class);
            //创建窗口
            self.h_window = winuser::CreateWindowExW(
                            0,
                            self.wnd_class_name.as_ptr(),
                            str_to_ws(self.title.as_str()).as_ptr(),
                            WS_OVERLAPPED | WS_VISIBLE | WS_CAPTION | WS_SYSMENU,
                            self.position.x, self.position.y,
                            self.width, self.height,
                            0 as HWND, 0 as HMENU, 0 as HINSTANCE, null_mut());
            //显示窗口
            winuser::ShowWindow(self.h_window, SW_SHOW);
            winuser::UpdateWindow(self.h_window);
        }
        0
    }

    // pub fn start_loop(&mut self){
    //     // 消息循环
    //     unsafe {
    //         loop {
    //             let pm = winuser::GetMessageW(&mut self.st_msg, 0 as HWND, 0, 0);
    //             if pm == 0 {
    //                 println!("消息循环结束!");
    //                 break;
    //             }
    //             winuser::TranslateMessage(&mut self.st_msg);
    //             winuser::DispatchMessageW(&mut self.st_msg);
    //         }
    //     }
    // }

    pub fn start_game_loop(&mut self, game_loop: unsafe fn(&mut Window)){
        // 消息循环
        while !self.done {
            unsafe {
                while winuser::PeekMessageW(&mut self.st_msg, 0 as HWND, 0, 0, PM_REMOVE) == 1 {
                    if self.st_msg.message == WM_QUIT {
                        self.done = true;
                        println!("消息循环结束!");
                    }else {
                        winuser::TranslateMessage(&mut self.st_msg);
                        winuser::DispatchMessageW(&mut self.st_msg);
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
    winuser::LoadImageW(0 as HINSTANCE,
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
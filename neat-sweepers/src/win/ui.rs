use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{RECT, HBRUSH, HDC, HGDIOBJ, HICON, HMENU, HPEN, HWND, LPPOINT, POINT};
use winapi::um::wingdi::{PS_SOLID, BLACK_PEN, HOLLOW_BRUSH, WHITE_PEN};
use winapi::um::winnt::{HANDLE, LPCWSTR};
use winapi::um::winuser::{
    IDI_APPLICATION, IMAGE_ICON, KEYEVENTF_KEYUP, MB_OK, MSG, PAINTSTRUCT, PM_REMOVE,
    SW_SHOW, WNDCLASSW, WS_CAPTION, WM_QUIT,
    WS_EX_TRANSPARENT, WS_OVERLAPPED, WS_SYSMENU, WS_VISIBLE,
};

use winapi::um::wingdi;
use winapi::um::winuser;

pub type Surface = HDC;
pub type Pen = HPEN;
pub type Brush = HBRUSH;
pub const KEY_CODE_SPACE: i32 = 0x20;
pub const KEY_CODE_LEFT: i32 = 0x25;
pub const KEY_CODE_RIGHT: i32 = 0x27;
pub const KEY_CODE_DOWN: i32 = 0x28;
pub const KEY_CODE_UP: i32 = 0x26;
pub const MM_ANISOTROPIC: i32 = 8;

pub fn solid_pen(width: i32, rgb: u32) -> Pen {
    unsafe { wingdi::CreatePen(PS_SOLID as i32, width, rgb) }
}

pub fn solid_brush(rgb: u32) -> Brush {
    unsafe { wingdi::CreateSolidBrush(rgb) }
}

pub fn select_pen(h_dc: HDC, pen: Pen) -> Pen {
    unsafe { wingdi::SelectObject(h_dc, pen as HGDIOBJ) as Pen }
}

pub fn select_brush(h_dc: HDC, brush: Brush) -> Brush {
    unsafe { wingdi::SelectObject(h_dc, brush as HGDIOBJ) as Brush }
}

pub fn select_hollow_brush(h_dc: HDC) -> Brush {
    unsafe { select_brush(h_dc, wingdi::GetStockObject(HOLLOW_BRUSH as i32) as Brush) }
}

pub fn select_black_pen(h_dc: HDC) -> Pen {
    unsafe { select_pen(h_dc, wingdi::GetStockObject(BLACK_PEN as i32) as Pen) }
}

pub fn select_white_pen(h_dc: HDC) -> Pen {
    unsafe { select_pen(h_dc, wingdi::GetStockObject(WHITE_PEN as i32) as Pen) }
}

pub fn move_to_ex(h_dc: HDC, x: f64, y: f64) {
    unsafe {
        wingdi::MoveToEx(h_dc, x as i32, y as i32, 0 as LPPOINT);
    }
}

pub fn move_to_ex_i32(h_dc: HDC, x: i32, y: i32) {
    unsafe {
        wingdi::MoveToEx(h_dc, x, y, 0 as LPPOINT);
    }
}

pub fn delete_pen(pen: Pen) {
    unsafe {
        wingdi::DeleteObject(pen as HGDIOBJ);
    }
}

pub fn delete_brush(b: Brush) {
    unsafe {
        wingdi::DeleteObject(b as HGDIOBJ);
    }
}

pub fn line_to(h_dc: HDC, x: f64, y: f64) {
    unsafe {
        wingdi::LineTo(h_dc, x as i32, y as i32);
    }
}

pub fn line_to_i32(h_dc: HDC, x: i32, y: i32) {
    unsafe {
        wingdi::LineTo(h_dc, x, y);
    }
}

pub fn text_out(h_dc: Surface, x: i32, y: i32, string: &str) {
    unsafe {
        let ws = str_to_ws(&string);
        let len = ws.len() - 1;
        wingdi::TextOutW(h_dc, x as i32, y as i32, ws.as_ptr(), len as i32);
    }
}

pub fn message_box(title: &str, msg: &str) {
    unsafe {
        winuser::MessageBoxW(
            null_mut(),
            str_to_ws(&title).as_ptr(),
            str_to_ws(&title).as_ptr(),
            MB_OK,
        );
    }
}

/** 画指定背景色的正方形 */
pub fn rectangle(h_dc: Surface, left: i32, top: i32, right: i32, bottom: i32, rgb: u32) {
    unsafe {
        //选择一个画笔画
        let pie_brush = wingdi::CreateSolidBrush(rgb);
        let old_brush = wingdi::SelectObject(h_dc, pie_brush as HGDIOBJ);
        wingdi::Rectangle(h_dc, left, top, right, bottom);
        wingdi::SelectObject(h_dc, old_brush as HGDIOBJ);
        wingdi::DeleteObject(pie_brush as HGDIOBJ);
    }
}

pub fn ellipse(h_dc: Surface, left: i32, top: i32, right: i32, bottom: i32) {
    unsafe {
        wingdi::Ellipse(h_dc, left, top, right, bottom);
    }
}

pub fn set_pixel(h_dc: Surface, x: i32, y: i32, rgb: u32) {
    unsafe {
        wingdi::SetPixel(h_dc, x, y, rgb);
    }
}

pub fn set_text_color(h_dc: Surface, rgb: u32) {
    unsafe {
        wingdi::SetTextColor(h_dc, rgb);
    }
}

pub fn set_map_mode(h_dc: Surface, mode: i32) {
    unsafe {
        wingdi::SetMapMode(h_dc, mode);
    }
}
pub fn set_viewport_ext_ex(h_dc: Surface, x: i32, y: i32) -> bool {
    unsafe { wingdi::SetViewportExtEx(h_dc, x, y, null_mut()) > 0 }
}
pub fn set_window_ext_ex(h_dc: Surface, x: i32, y: i32) -> bool {
    unsafe { wingdi::SetWindowExtEx(h_dc, x, y, null_mut()) > 0 }
}
pub fn set_viewport_org_ex(h_dc: Surface, x: i32, y: i32) -> bool {
    unsafe { wingdi::SetViewportOrgEx(h_dc, x, y, null_mut()) > 0 }
}

pub fn set_bk_mode_transparent(h_dc: Surface) {
    unsafe {
        wingdi::SetBkMode(h_dc, WS_EX_TRANSPARENT as i32);
    }
}

/** 字符串转换成双字 0结尾的数组 */
pub fn str_to_ws(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

pub fn key_down(vk_code: i32) -> bool {
    unsafe { winuser::GetAsyncKeyState(vk_code) as i32 & 0x8000 != 0 }
}

pub fn key_down_event(vk_code: u8) {
    unsafe {
        winuser::keybd_event(vk_code, 0, 0, 0);
    }
}

pub fn key_up_event(vk_code: u8) {
    unsafe {
        winuser::keybd_event(vk_code, 0, KEYEVENTF_KEYUP, 0);
    }
}

//窗口
pub struct Window {
    position: Point,
    title: String, //窗口名称
    width: i32,
    height: i32,
    wnd_class: WNDCLASSW,
    wnd_class_name: Vec<u16>,
    h_window: HWND,
    st_msg: MSG,
    done: bool,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            winuser::UnregisterClassW(self.wnd_class_name.as_ptr(), self.wnd_class.hInstance);
        }
    }
}

impl Window {
    pub fn new(
        title: String,
        width: i32,
        height: i32,
        position: Point,
        icon_file_name: Option<String>,
        window_proc: unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT,
    ) -> Window {
        let wnd_class_name = str_to_ws("super_window");

        //创建并窗口类
        let wnd_class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: match icon_file_name.as_ref() {
                Some(file_name) => unsafe { load_image(file_name.as_str(), IMAGE_ICON) as HICON },
                _ => unsafe { winuser::LoadIconW(0 as HINSTANCE, IDI_APPLICATION) },
            },
            hCursor: unsafe { winuser::LoadCursorW(0 as HINSTANCE, IDI_APPLICATION) },
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: wnd_class_name.as_ptr(),
        };

        Window {
            position: position,
            title: title,
            width: width,
            height: height,
            wnd_class: wnd_class,
            wnd_class_name: wnd_class_name,
            h_window: 0 as HWND,
            st_msg: MSG {
                hwnd: 0 as HWND,
                message: 0 as UINT,
                wParam: 0 as WPARAM,
                lParam: 0 as LPARAM,
                time: 0 as DWORD,
                pt: POINT { x: 0, y: 0 },
            },
            done: false,
        }
    }

    pub fn show(&mut self) -> i32 {
        unsafe {
            //注册窗口类
            winuser::RegisterClassW(&self.wnd_class);
            //创建窗口
            self.h_window = winuser::CreateWindowExW(
                0,
                self.wnd_class_name.as_ptr(),
                str_to_ws(self.title.as_str()).as_ptr(),
                WS_OVERLAPPED | WS_VISIBLE | WS_CAPTION | WS_SYSMENU,
                self.position.x,
                self.position.y,
                self.width,
                self.height,
                0 as HWND,
                0 as HMENU,
                0 as HINSTANCE,
                null_mut(),
            );
            //显示窗口
            winuser::ShowWindow(self.h_window, SW_SHOW);
            winuser::UpdateWindow(self.h_window);
        }
        0
    }

    pub fn start_loop(&mut self) {
        // 消息循环
        unsafe {
            loop {
                let pm = winuser::GetMessageW(&mut self.st_msg, 0 as HWND, 0, 0);
                if pm == 0 {
                    println!("消息循环结束!");
                    break;
                }
                winuser::TranslateMessage(&mut self.st_msg);
                winuser::DispatchMessageW(&mut self.st_msg);
            }
        }
    }

    pub fn start_game_loop(&mut self, game_loop: unsafe fn(&mut Window)) {
        // 消息循环
        while !self.done {
            unsafe {
                while winuser::PeekMessageW(&mut self.st_msg, 0 as HWND, 0, 0, PM_REMOVE) == 1 {
                    if self.st_msg.message == WM_QUIT {
                        self.done = true;
                        println!("消息循环结束!");
                    } else {
                        winuser::TranslateMessage(&mut self.st_msg);
                        winuser::DispatchMessageW(&mut self.st_msg);
                    }
                }
                game_loop(self);
            }
        }
    }

    pub fn close(&mut self) {
        self.done = true;
    }

    pub fn get_handle(&self) -> HWND {
        self.h_window
    }
}

unsafe fn load_image(file_name: &str, file_type: UINT) -> HANDLE {
    winuser::LoadImageW(
        0 as HINSTANCE,
        str_to_ws(file_name).as_ptr(), //图片文件名
        file_type, //IMAGE_BITMAP: UINT = 0; 装载位图 IMAGE_ICON: UINT = 1; 装载位图
        //IMAGE_CURSOR = 2 光标
        0,
        0, //资源宽高自动获取
        0x00000010,
    ) // as LR_LOADFROMFILE
}

pub fn new_rect() -> RECT {
    RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    }
}

pub fn new_paint_st() -> PAINTSTRUCT {
    PAINTSTRUCT {
        hdc: 0 as HDC,
        fErase: 0,
        rcPaint: new_rect(),
        fRestore: 0,
        fIncUpdate: 0,
        rgbReserved: [0; 32],
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x;
        self.y = source.y;
    }
}

pub fn rgb(red: i32, green: i32, blue: i32) -> u32 {
    red as u32 | (green as u32) << 8 | (blue as u32) << 16
}

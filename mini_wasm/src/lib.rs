use std::mem;

//导入的JS帮助函数
extern "C" {
    pub fn _console_log(text: *const u8, len: usize);
}

pub fn console_log(msg: &str) {
    unsafe {
        _console_log(msg.as_ptr(), msg.len());
    }
}

#[no_mangle]
pub fn on_connect() {
    console_log("Rust:on_connect 调用");
}


#[no_mangle]
pub fn alloc(size: usize) -> *const u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub fn start() {
    console_log("Rust:start!!!!");
}
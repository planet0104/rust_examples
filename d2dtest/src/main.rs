use winit::dpi::LogicalSize;
use winit::{WindowEvent, Event};
#[cfg(target_os = "windows")]
#[macro_use]
extern crate native_windows_gui as nwg;

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(500.0, 400.0))
        .with_title("Test")
        .build(&events_loop)
        .unwrap();

    #[cfg(target_os = "windows")]
    {
        use winit::os::windows::WindowExt;
        let hwnd = window.get_hwnd();
        let cavnas = nwg_canvas!( parent=hwnd; size=(500, 400));

    }

    let mut exit = false;
    while !exit{
        events_loop.poll_events(|event| {
            // println!("{:?}", event);
            match event {
                Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } =>{
                    exit = true;
                    winit::ControlFlow::Break
                }
                _ => winit::ControlFlow::Continue,
            };
        });
    }
}

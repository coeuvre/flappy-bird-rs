extern crate glutin;
extern crate libc;

use glutin::GlContext;

pub mod gfx;
pub mod math;

use gfx::Graphics;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Flappy Bird")
        .with_dimensions(288, 512);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();

        // gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        // gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut gfx = Graphics::new(|symbol| gl_window.get_proc_address(symbol) as *const _);
    let background_texture = gfx.load_texture("assets/sprites/background_day.png")
        .unwrap();

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(glutin::VirtualKeyCode::Escape) {
                        running = false;
                    }
                }
                _ => (),
            },
            _ => (),
        });

        gfx.clear();

        gfx.draw_texture(&background_texture);

        gl_window.swap_buffers().unwrap();
    }
}

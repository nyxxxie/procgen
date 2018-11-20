extern crate sdl2;
extern crate gl;

use std::ffi::{ CString };
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WINDOW_WIDTH: u32 = 900;
const WINDOW_HEIGHT: u32 = 700;

fn main() {
    println!("Hello, world!");

    /* Init sdl2 */
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    /* Set up opengl context for window */
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    /* Create window */
    let window = video_subsystem.window("Renderer", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    /* Init opengl */
    let _gl_ctx = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    }

    /* Main game loop */
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'main_loop: loop {
        /* Listen for window events */
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown{ keycode: Option::Some(Keycode::Escape), ..} |
                Event::Quit{..} => {
                    break 'main_loop;
                },
                _ => {},
            }
            // handle user input here
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window()
    }
}

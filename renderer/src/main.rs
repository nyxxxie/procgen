extern crate sdl2;
extern crate gl;

pub mod engine;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;

// TODO: make a camera class.  Add film controls, flight controls, and point travel with
// easing.

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
    let window = video_subsystem.window("Renderer", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    /* Init opengl */
    let _gl_ctx = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    }

    /* Create shader programs */
    let tri_shader = engine::shader::Program::new_basic(
        include_str!("engine/shaders/triangle.vert"),
        include_str!("engine/shaders/triangle.frag")).unwrap();

    let quad_shader = engine::shader::Program::new_basic(
        include_str!("engine/shaders/quad.vert"),
        include_str!("engine/shaders/quad.frag")).unwrap();

    /* Create vertices */
    let vertices: Vec<f32> = vec![
        // positions    colors
       -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
    ];
	
	let tri_mesh = engine::mesh::MeshBuilder::new()
		.vertex_data(&vertices)
		.attribute(0, 3)
		.attribute(1, 3)
		.shader(tri_shader)
		.build();

	let quad_mesh = engine::mesh::MeshBuilder::new_quad()
		.shader(quad_shader)
		.build();

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

        /* Draw the meshes */
		quad_mesh.draw_triangles();
		tri_mesh.draw_triangles();

        window.gl_swap_window();
    }
}

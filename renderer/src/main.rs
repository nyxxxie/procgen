extern crate sdl2;
extern crate glm;
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
        .input_grabbed()
        .build()
        .unwrap();

    /* Hide mouse cursor, since we grab it when we create the window */
    sdl_ctx.mouse().show_cursor(false);

    /* Init opengl */
    let _gl_ctx = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
    }

    /* Set up the mouse cursor for fps-like controls */
    //sdl_ctx.mouse().show_cursor(false);
    sdl_ctx.mouse().capture(true);

    /* Create shader programs */
    let colored_shader = engine::shader::Program::new_basic(
        include_str!("engine/shaders/basic/colored.vert"),
        include_str!("engine/shaders/basic/colored.frag")).unwrap();

    let uncolored_shader = engine::shader::Program::new_basic(
        include_str!("engine/shaders/basic/uncolored.vert"),
        include_str!("engine/shaders/basic/uncolored.frag")).unwrap();

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
		.shader(colored_shader)
		.build();

	let quad_mesh = engine::mesh::MeshBuilder::new_quad()
		.shader(uncolored_shader)
		.build();

    /* Create camera */
    let camera = engine::camera::Camera::new(glm::vec3(1.0, 1.0, 1.0));

    /* Main game loop */
    //let mut delta_time = 0.0f;
    //let mut last_frame = 0.0f;
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'main_loop: loop {
        /* Calc time delta */
        //let current_time = something;
        //delta_time = current_frame - last_frame;
        //last_frame = current_frame;

        /* Listen for window events */
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown{ keycode: Option::Some(Keycode::Escape), ..} |
                Event::Quit{..} => {
                    break 'main_loop;
                },
                _ => {},
            }

            camera.process_event(0.0, &event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        //println!("viematrix:\n{:?}", camera.get_view_matrix());

        /* Draw the meshes */
		quad_mesh.draw_triangles();
		tri_mesh.draw_triangles();

        window.gl_swap_window();
    }
}

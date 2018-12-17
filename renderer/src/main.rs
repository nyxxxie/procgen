extern crate sdl2;
extern crate glm;
extern crate gl;

pub mod engine;

use std::time;
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

    /* Hide mouse cursor and lock it to center of screen */
    sdl_ctx.mouse().show_cursor(false);
    sdl_ctx.mouse().set_relative_mouse_mode(true);

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
    let object_shader = engine::shader::Program::new_basic(
        include_str!("engine/shaders/basic/object.vert"),
        include_str!("engine/shaders/basic/object.frag")).unwrap();

	let cube_mesh = engine::mesh::MeshBuilder::new()
		.vertex_data(&[
           -0.5, -0.5, -0.5, // -Z face
            0.5, -0.5, -0.5,
            0.5,  0.5, -0.5,
           -0.5,  0.5, -0.5,
           -0.5, -0.5,  0.5, // +Z face
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,
           -0.5,  0.5,  0.5,
           -0.5, -0.5, -0.5,  // -X face
           -0.5,  0.5, -0.5,
           -0.5,  0.5,  0.5,
           -0.5, -0.5,  0.5,
            0.5, -0.5, -0.5,  // +X face
            0.5,  0.5, -0.5,
            0.5,  0.5,  0.5,
            0.5, -0.5,  0.5,
           -0.5, -0.5, -0.5,  // -Y face
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
           -0.5, -0.5,  0.5,
           -0.5,  0.5, -0.5,  // +Y face
            0.5,  0.5, -0.5,
            0.5,  0.5,  0.5,
           -0.5,  0.5,  0.5,
        ])
		.indices(&[
            0,  1,  2,  // first triangle is index 0, 1, and 2 in the list of vertices
            0,  2,  3,
            4,  5,  6,  // second quad
            4,  6,  7,
            8,  9,  10, // third quad
            8,  10, 11,
            12, 13, 14, // fourth quad
            12, 14, 15,
            16, 17, 18, // fifth quad
            16, 18, 19,
            20, 21, 22, // sixth quad
            20, 22, 23
        ])
		.attribute(0, 3)
		.build();

    /* Create camera */
    let mut camera = engine::camera::Camera::new(glm::vec3(1.0, 1.0, 1.0), WINDOW_WIDTH, WINDOW_HEIGHT);

    /* Initialize tick stuff */
    let start_time = time::SystemTime::now();
    let mut delta_time: f32 = 0.0;
    let mut last_frame: f32 = 0.0;

    /* Main game loop */
    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'main_loop: loop {
        /* Calc time delta */
        let elapsed = start_time.elapsed().unwrap();
        let current_frame = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32) / 1_000_000_000f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        /* Listen for window events */
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown{ keycode: Option::Some(Keycode::Escape), ..} |
                Event::Quit{..} => {
                    break 'main_loop;
                },
                _ => {},
            }

            camera.process_event(delta_time, &event);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        /* Calc modelview matrix */
        let mut modelview = camera.get_view_matrix();
        modelview = glm::ext::translate(&modelview, glm::vec3(0.0, 0.0, -4.0));

        /* Set uniforms for the cube's shader */
        object_shader.activate();

        let modelview_loc = object_shader.get_uniform("modelview").unwrap();
        let projection_loc = object_shader.get_uniform("projection").unwrap();

        object_shader.set_uniform_mat4f(modelview_loc, &modelview);
        object_shader.set_uniform_mat4f(projection_loc, &camera.get_projection_matrix());

        /* Draw the meshes */
		cube_mesh.draw_triangles();

        window.gl_swap_window();
    }
}

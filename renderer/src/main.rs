extern crate sdl2;
extern crate gl;

pub mod engine;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use gl::types::{ GLint, GLuint, GLsizeiptr, GLvoid };

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 700;

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

    /* Create shader program */
    let shader = engine::shader::Program::new_basic(
        include_str!("engine/shaders/triangle.vert"),
        include_str!("engine/shaders/triangle.frag")).unwrap();

    /* Create and bind vertex array object for our mesh */
    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    /* Create vertices */
    // TODO: make a Mesh struct (accept arbitrary vertex and/or index data),
    // set attributes by either location or by name, add functions to bind,
    // unbind, and draw the mesh
    //
    // Alternatively, create a Vertex object where we can set position, color,
    // texture coords, etc.  These will automaticaly be linked to attributes.
    // Maybe expose the first raw interface and then put this on top?
    //
    // Also, create a premade mesh generator (colored cube, textured cube, cylinder, quad, etc)

    // TODO: make a camera class.  Add film controls, flight controls, and point travel with
    // easing.
    let vertices: Vec<f32> = vec![
       -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0,  0.5, 0.0,
    ];

    /* Create a buffer and put vertices inside of it */
    let mut vbo: GLuint = 0;
    unsafe {
        /* Create and bind buffer object to put vertices in.  Since we bound
         * the vertex array object above, this buffer will be associated with
         * it. */
        gl::GenBuffers(1, &mut vbo);  // create 1 new buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);  // make buffer active
        gl::BufferData(
            gl::ARRAY_BUFFER,  // target
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,  // size of data
            vertices.as_ptr() as *const GLvoid,  // pointer to data
            gl::STATIC_DRAW);  // usage

        /* Tell buffer about attributes */
        gl::EnableVertexAttribArray(0);  // attrib layout (location = 0)
        gl::VertexAttribPointer(
            0,  // attribute location
            3,  // number of components for this attribute
            gl::FLOAT,  // attribute data type
            gl::FALSE,  // normalized
            ( 3 * std::mem::size_of::<f32>()) as GLint,  // offset between consecutive attributes
            std::ptr::null());  // offset of first component
    }

    /* Finish modifying vao by unbinding everything */
    unsafe {
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
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

        /* Draw the triangle */
        shader.set_used();
        unsafe {
            /* Bind vao.  This will automatically bind the vbo too */
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,  // mode
                0,  // starting index
                3);  // number of indices to be rendered
        }

        window.gl_swap_window()
    }
}

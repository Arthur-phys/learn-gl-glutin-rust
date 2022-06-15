#![allow(non_upper_case_globals)]

use glutin::{ContextBuilder, GlRequest, Api, GlProfile}; // Create OpenGL context and use 3.3 version
use glutin::event_loop::{ControlFlow, EventLoop}; // Event loop
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glutin::event::{Event, WindowEvent}; // Event creation for updating view
use glutin::window::WindowBuilder; // Needed to create a window
use glutin::dpi::PhysicalSize; // to specify size of window
use std::os::raw::c_void;
use std::{ptr,str,mem};
use crate::shader::*;
// Vertex should be on NDC (Normalized Device Coordinates). That means between -1 an 1 (as float)
// This is data for a triangle.
const vertices: [f32; 18] = [
     // positions    // colors
    -0.5, 0.5, 0.0, 1.0, 0.0, 0.0,
     0.5, 0.5, 0.0, 0.0, 1.0, 0.0,
     0.0, -0.5, 0.0, 0.0, 0.0, 1.0

];

pub fn upside_down() {
    // ################
    // Initialization #
    // ################
    let window_builder = WindowBuilder::new().with_title("Dzahui").with_inner_size(PhysicalSize {height: 600, width: 800}).with_resizable(true);    
    let event_loop = EventLoop::new(); // creating event loop
    let opengl_version = GlRequest::Specific(Api::OpenGl, (3,3)); // Specific OpenGL version (same as book)
    let new_context = ContextBuilder::new().
    with_gl(opengl_version).with_gl_profile(GlProfile::Core).build_windowed(window_builder, &event_loop).unwrap(); // core GL
    // future compatible functions. Not backwards compatible (no previous versions of openGL)
    let new_context = unsafe { new_context.make_current().unwrap() };
    // Loading of opengl functions right below
    gl::load_with(&|s: &str| {
        new_context.get_proc_address(s)
    });
    let vertex_path = "/home/Arthur/Tesis/learn-opengl-glutin-gl-rust/src/_1_getting_started/shaders/vertex_shader_10.vs";
    let fragment_path = "/home/Arthur/Tesis/learn-opengl-glutin-gl-rust/src/_1_getting_started/shaders/fragment_shader_10.fs";
    let shader_program = Shader::new(vertex_path, fragment_path);
    // ################
    // Initialization #
    // ################
    // #####################
    // Binding of vertices #
    // #####################
    // generating a VBO (Vertex Buffer Object - Store a large number of vertices in the graphics card)
    // An OpenGL object is a subset of the complete state of OpenGL
    let mut vbo: u32 = 0;
    let mut vao: u32 = 0; // vertex array object to store all config and just bind to view every time we have to work with it
    // substitutes using all the calls to VBO
    unsafe {
        gl::GenVertexArrays(1,&mut vao); // Generate a VAO to link to buffers
        gl::GenBuffers(1, &mut vbo); // generates a buffer in GPU.
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER,vbo); // binding buffer to specific type ARRAY_BUFFER
        gl::BufferData(gl::ARRAY_BUFFER,(vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,gl::STATIC_DRAW); // double casting to raw pointer of c_void
        // this part is copying the vertices into the buffer (or at least telling it where to find them (given size and first element reference)).
        // STATIC_DRAW means data only set once but used many times. Ther are another two configurations: STREAM_DRAW and DYNAMC_DRAW. ONe is used when data
        // is accesed a few times and set only once, the other serves when data is used a lot an changed a lot.
        // NOw we have to tell OpenGL how to parse the vertex data. Since it is a 64-ibt floating point array tightly packed. we do as it follows
        gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,6*mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0); // Enabling vertex atributes giving vertex location (setup in vertex shader).
        gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,6*mem::size_of::<GLfloat>() as GLsizei,(3 * mem::size_of::<GLfloat>()) as *const c_void);
        // first parameter is pointer to attrribute (location used by shader), second is number of coordinates, third is type of data,
        // fourth is if coordinates should be normalized, fifth is if byte offset (or size) between vectors and sixth is offset of first component.
        gl::EnableVertexAttribArray(1);
    }
    // #####################
    // Binding of vertices #
    // #####################
    
    event_loop.run(move |event, _, control_flow| {
        println!("{:?}",event);
        match event {
            Event::LoopDestroyed => (), // subscribing to events occurs here
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(physical_size) => new_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {input, is_synthetic, ..} => {
                    if !is_synthetic && input.scancode == 1 {
                        println!("\n Perrito supergordito \n"); // testing keyboard input
                        // check keyboard scancodes at: https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
                        // also is possible to check event log (after the event_loop.run)
                        *control_flow = ControlFlow::Exit;
                    } else if input.scancode == 32 { // letter d
                        println!("\n Do a barrel roll! \n"); // Star fox's Peppy line on N64
                    }
                }
                _ => ()
            },
            _ => ()
        }
        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            // draw our first triangle
            shader_program.use_shader();
            gl::BindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // glBindVertexArray(0); // no need to unbind it every time
        }
        new_context.swap_buffers().unwrap(); //needed to cahnge old and new buffer and redraw
    });
}

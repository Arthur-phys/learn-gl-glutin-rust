#![allow(non_upper_case_globals)]

use std::ffi::CString;
use glutin::{ContextBuilder, GlRequest, Api, GlProfile}; // Create OpenGL context and use 3.3 version
use glutin::window::WindowBuilder; // Needed to create a window
use glutin::event::{Event, WindowEvent}; // Event creation for updating view
use glutin::event_loop::{ControlFlow, EventLoop}; // Event loop
use glutin::dpi::PhysicalSize; // to specify size of window
use std::os::raw::c_void;
use gl::types::{GLint, GLchar, GLfloat, GLsizei, GLsizeiptr};
use std::{ptr,str,mem};
use gl;

// We give the vertices as a constant array
const vertices: [f32;15] = [
    -0.5,0.0,0.0,
    0.0,0.0,0.0,
    0.0,0.5,0.0,
    0.5,0.0,0.0,
    0.5,0.5,0.0
];
// We need to specify the order to draw the vertices (such that we end up drawing triangles)
const indices: [u32; 6] = [
    0, 1, 2,
    1, 3, 4
];

// the vertex shder is stored in a variable for now. It is written in GLSL (OpenGL Shading Language)
// it defines verices (as per the name).
const vertex_shader_source: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#; // raw string literal. Writing without escaping

//the fragment shader is also stored in a variable. It defines the color of the pixels (after rasterization).
const fragment_shader_source: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub fn traingles_together() {

    //###############################################################################################
    //########################## STUFF FROM PREVIOUS EXCERSISE ######################################
    //###############################################################################################

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
    
    // Creation of a vertex shader object
    let vertex_shader: u32; // needed for referencing shader
    let c_str_code = CString::new(vertex_shader_source.as_bytes()).unwrap(); // Neede to call function ShaderSource
    let mut shader_log: Vec<u8> = Vec::with_capacity(512);
    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER); // creates the shader based on the type passed and 'stores it' on vertex_shader
        gl::ShaderSource(vertex_shader,1,&c_str_code.as_ptr(),ptr::null()); // attaching shader code to shader object
        // 1 is specifiying how many strings are passed as arguments (only one)
        gl::CompileShader(vertex_shader); // comṕiling shader
        // Check if shader was successfully created
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader,gl::COMPILE_STATUS, &mut success); //changes variable success depending on wether the shader was compiled correctly
        if success == gl::FALSE as GLint {
            gl::GetShaderInfoLog(vertex_shader,512,ptr::null_mut(),shader_log.as_mut_ptr() as *mut GLchar);
            println!("FAILURE IN VERTEX SHADER COMPILATION!!!: {}", str::from_utf8(&shader_log).unwrap());
        }
    }
    
    // Creation of a fragment shader object
    let fragment_shader: u32;
    let c_str_fragment_shader = CString::new(fragment_shader_source.as_bytes()).unwrap();
    let mut fragment_log: Vec<u8> = Vec::with_capacity(512);
    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader,1,&c_str_fragment_shader.as_ptr(),ptr::null());
        gl::CompileShader(fragment_shader);
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(fragment_shader,gl::COMPILE_STATUS, &mut success); //changes variable success depending on wether the shader was compiled correctly
        if success == gl::FALSE as GLint {
            gl::GetShaderInfoLog(fragment_shader,512,ptr::null_mut(),fragment_log.as_mut_ptr() as *mut GLchar);
            println!("FAILURE IN FRAGMENT SHADER COMPILATION!!!: {}", str::from_utf8(&shader_log).unwrap());
        }
    }
    
    // Creation of a shader program. That which links all shaders to be used. The shaders have to be in correct order accroding to graphics pipeline.
    let program_shader: u32;
    let mut program_log: Vec<u8> = Vec::with_capacity(512);
    unsafe {
        program_shader = gl::CreateProgram(); // Creation of program shader
        gl::AttachShader(program_shader,vertex_shader); // Attaching first vertex shader (since its fir4st in pipeline)
        gl::AttachShader(program_shader,fragment_shader); // Much later comes fragment shader.
        gl::LinkProgram(program_shader); // NOw we link them together (after attaching)
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program_shader,gl::LINK_STATUS, &mut success); //changes variable success depending on wether the shader was compiled correctly
        if success == gl::FALSE as GLint {
            gl::GetProgramInfoLog(program_shader,512,ptr::null_mut(),program_log.as_mut_ptr() as *mut GLchar);
            println!("FAILURE IN PROGRAM SHADER COMPILATION!!!: {}", str::from_utf8(&program_log).unwrap());
        }
        gl::DeleteShader(vertex_shader); // Once linked, no longer neededd
        gl::DeleteShader(fragment_shader); // Once linked, no longer neededd
    }
    
    //###############################################################################################
    //########################## STUFF FROM PREVIOUS EXCERSISE ######################################
    //###############################################################################################
    
    let mut vbo: u32 = 0;
    let mut vao: u32 = 0;
    let mut ebo = 0; // Element buffer object. This stores indices and not vertices, unlike vbo.
    unsafe {
        // The order of binding is really important
        // Specially for the VertexArray, since it takes the first vertex buffer object and element buffer object that are bound after it
        gl::GenVertexArrays(1,&mut vao); // Generate a VAO to link to buffers
        gl::BindVertexArray(vao); // since it is bound first, it binds to the EBO and VBO (because they are the only ones being bound after it)
        gl::GenBuffers(1, &mut vbo); // generates a buffer in GPU.
        gl::GenBuffers(1, &mut ebo); // Generating buffer for ebo
        gl::BindBuffer(gl::ARRAY_BUFFER,
            vbo); // binding buffer to specific type ARRAY_BUFFER
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW); // double casting to raw pointer of c_void
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER,
            ebo); // We need to specify the kind of buffer we want to bind
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &indices[0] as *const u32 as *const c_void,
            gl::STATIC_DRAW);
        gl::VertexAttribPointer(0,
            3,
            gl::FLOAT,
        gl::FALSE,
        3*mem::size_of::<GLfloat>() as GLsizei,
        ptr::null());
        gl::EnableVertexAttribArray(0); // Enabling vertex atributes giving vertex location (setup in vertex shader).
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); // comment to see them filled instead of only the lines that form them
    }

    event_loop.run(move |event, _, control_flow| {
        println!("{:?}",event);
        *control_flow = ControlFlow::Wait;

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
            
            // draw two triangles next to each other
            gl::UseProgram(program_shader);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES,6,gl::UNSIGNED_INT,ptr::null());
        }
        new_context.swap_buffers().unwrap(); //needed to cahnge old and new buffer and redraw
    });
}
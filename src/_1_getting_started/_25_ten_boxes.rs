#![allow(non_upper_case_globals)]

use cgmath::{self, Matrix4, Deg, Vector3, Matrix};
use glutin::{ContextBuilder, GlRequest, Api, GlProfile}; // Create OpenGL context and use 3.3 version
use glutin::event_loop::{ControlFlow, EventLoop}; // Event loop
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use glutin::event::{Event, WindowEvent}; // Event creation for updating view
use glutin::window::WindowBuilder; // Needed to create a window
use glutin::dpi::PhysicalSize; // to specify size of window
use std::os::raw::c_void;
use image;
use std::{vec::Vec,ptr,str,mem,time::Instant};
use crate::shader::*;

const vertices: [f32;64] = [
     // positions   // colors      // textures
     // front-face 
      0.5,  0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
      0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
     -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
     -0.5,  0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0,
     // back-face 
      0.5,  0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
      0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0,
     -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0,
     -0.5,  0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0 
];

const indices: [u32;36] = [
     //front-face
     0, 1, 3,
     3, 2, 1,
     //up-face
     0, 4, 3,
     3, 7, 4,
     //down-face
     1, 5, 2,
     2, 6, 5,
     //left-face
     3, 7, 2,
     2, 6, 7,
     //right-face
     0, 4, 1,
     1, 5, 4,
     //back-face
     4, 5, 7,
     7, 6, 5,
];

pub fn ten_boxes() {

     // #################
     // Creating matrix #
     // #################

     let view_mat: Matrix4<f32> = Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0));
     let proj_mat: Matrix4<f32> = cgmath::perspective(Deg(45.0), 800.0/600.0, 0.1, 100.0);
     let translate_model_vectors: [Vector3<f32>;10] = [
        Vector3::new(0.0,0.0,0.0),
        Vector3::new(2.0,5.0,-15.0),
        Vector3::new(-1.5,-2.2,-2.5),
        Vector3::new(-3.8,-2.0,-12.3),
        Vector3::new(2.4,-0.4,-3.5),
        Vector3::new(-1.7,3.0,-7.5),
        Vector3::new(1.3,-2.0,-2.5),
        Vector3::new(1.5,2.0,-2.5),
        Vector3::new(1.5,0.2,-1.5),
        Vector3::new(-1.3,1.0,-1.5),
     ];
     let mut translate_model_mats: Vec<Matrix4<f32>> = Vec::new();
     for i in 0..10 {
        let model_mat: Matrix4<f32> = Matrix4::from_translation(translate_model_vectors[i]);
        translate_model_mats.push(model_mat);
   }
     
     // #################
     // Creating matrix #
     // #################
     
     // #################
     // Image obtaining #
     // #################
 
     let img = image::open("/home/Arthur/Tesis/learn-opengl-glutin-gl-rust/src/_1_getting_started/images/container.jpg").unwrap();
     let height = img.height();
     let width = img.width();
     let img_vec: Vec<u8> = img.into_bytes();
 
     let img_2 = image::open("/home/Arthur/Tesis/learn-opengl-glutin-gl-rust/src/_1_getting_started/images/awesomeface.png").unwrap().flipv();
     let height_2 = img_2.height();
     let width_2 = img_2.width();
     let img_vec_2: Vec<u8> = img_2.into_bytes();
 
     // #################
     // Image obtaining #
     // #################
 
     // ################
     // Initialization #
     // ################
 
     let window_builder = WindowBuilder::new().
         with_title("Dzahui").
         with_inner_size(PhysicalSize {height: 600, width: 800}).
         with_resizable(true);
 
     let event_loop = EventLoop::new(); // creating event loop
 
     let opengl_version = GlRequest::Specific(Api::OpenGl, (3,3)); // Specific OpenGL version (same as book)
 
     let new_context = ContextBuilder::new().
         with_gl(opengl_version).
         with_gl_profile(GlProfile::Core).
         build_windowed(window_builder, &event_loop).
         unwrap(); // core GL
 
     // future compatible functions. Not backwards compatible (no previous versions of openGL)
     let new_context = unsafe { 
         new_context.make_current().unwrap() 
     };
 
     // Loading of opengl functions right below
     gl::load_with(&|s: &str| {
         new_context.get_proc_address(s)
     });
 
     // creation of shader_program
     let vertex_path = "/home/Arthur/Tesis/learn-opengl-glutin-gl-rust/src/_1_getting_started/shaders/vertex_shader_23.vs";
     let fragment_path = "/home/Arthur/Tesis/learn-opengl-glutin-gl-rust/src/_1_getting_started/shaders/fragment_shader_23.fs";
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
     let mut ebo: u32 = 0;
     let mut texture: u32 = 0; // texture variable to bind to.
     let mut texture_2: u32 = 0; // awesomeface texture
 
     unsafe {
        gl::GenVertexArrays(1,&mut vao); // Generate a VAO to link to buffers
        gl::GenBuffers(1, &mut vbo); // generates a buffer in GPU.
        gl::GenBuffers(1,&mut ebo);
        
        // binding of vao. Since it's only one, it can be done once here.
        gl::BindVertexArray(vao); // vertex array binding
        
        gl::BindBuffer(gl::ARRAY_BUFFER,vbo); // binding buffer to specific type ARRAY_BUFFER
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW); // double casting to raw pointer of c_void

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &indices[0] as *const u32 as *const c_void, 
            gl::STATIC_DRAW);
        
        // vertex coordinates
        gl::VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,8*mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0); // Enabling vertex atributes giving vertex location (setup in vertex shader).
        // color RGB
        gl::VertexAttribPointer(1,3,gl::FLOAT,gl::FALSE,8*mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1); // Enabling vertex atributes giving vertex location (setup in vertex shader).
        // texture coordinates
        gl::VertexAttribPointer(2,2,gl::FLOAT,gl::FALSE,8*mem::size_of::<GLfloat>() as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2); // Enabling vertex atributes giving vertex location (setup in vertex shader).
         
        // generate texture
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0); // activate texture zero (one out of 16)
        gl::BindTexture(gl::TEXTURE_2D,texture); // binding to texture 2d
        // create texture and generate mipmaps
        gl::TexImage2D(gl::TEXTURE_2D, // Texture target is 2D since we created a texture for that
            0, // Mipmap level 0 which is default. Otherwise wue could specify levels and change it
            gl::RGB as i32, // Image is given as values of RGB
            width as i32,
            height as i32,
            0, // Legacy sutff not explained
            gl::RGB, // Format of the image (this is the actual format)
            gl::UNSIGNED_BYTE, // RGB values are given as chars
            &img_vec[0] as *const u8 as *const c_void); // Pointer to first element of vector
        gl::GenerateMipmap(gl::TEXTURE_2D); // generate mipmap for texture 2d (when object is far or close)
        // texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); //how to wrap in s coordinate
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); // how to wrap in t coordinate
        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32); // when texture is small, scall using linear
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); // when texture is big, scall using linear
         
         // generate second texture
         gl::GenTextures(1, &mut texture_2);
         gl::ActiveTexture(gl::TEXTURE1); // activate texture one (one out of 16)
         gl::BindTexture(gl::TEXTURE_2D,texture_2);
         // create texture and generate mipmaps
         gl::TexImage2D(
             gl::TEXTURE_2D, // Texture target is 2D since we created a texture for that
             0, // Mipmap level 0 which is default. Otherwise wue could specify levels and change it
             gl::RGB as i32, // Image is given as values of RGB
             width_2 as i32,
             height_2 as i32,
             0, // Legacy sutff not explained
             gl::RGBA, // Format of the image (this is the actual format)
             gl::UNSIGNED_BYTE, // RGB values are given as chars
             &img_vec_2[0] as *const u8 as *const c_void); // Pointer to first element of vector
        gl::GenerateMipmap(gl::TEXTURE_2D); // generate mipmap for texture 2d (when object is far or close)

        // texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); //how to wrap in s coordinate
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); // how to wrap in t coordinate
        // texture filtering
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32); // when texture is small, scall using linear
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); // when texture is big, scall using linear
        
        // ENABLE DEPTH TEST to know when to draw on top of something
        gl::Enable(gl::DEPTH_TEST);

        // Use shader
        shader_program.use_shader();
        // we need to tell the shader which variables are associated with which textures
        shader_program.set_int("ourTexture",0); // use TEXTURE0
        shader_program.set_int("secondTexture",1); // use TEXTURE1
        // if we invert them, the shader fragment will also invert the percentage of each one present in the final pixels

        // Use matrix
        shader_program.set_mat4("view", &view_mat);
        shader_program.set_mat4("projection", &proj_mat);
     }
     
     // #####################
     // Binding of vertices #
     // #####################
     
     // ############
     // Event Loop #
     // ############
     let time = Instant::now(); //time to get degrees
     event_loop.run(move |event, _, control_flow| {
          println!("{:?}",event);
          
          match event { // subscribing to events occurs here
               Event::LoopDestroyed => (),
               
               // window events
               Event::WindowEvent {event, ..} => match event {
                    // window events
                    WindowEvent::Resized(physical_size) => new_context.resize(physical_size),
                    
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
 
                    WindowEvent::KeyboardInput {input, is_synthetic, ..} => {
                         
                         // esc key
                         if !is_synthetic && input.scancode == 1 {
                              *control_flow = ControlFlow::Exit;
                              
                              // D key
                         } else if input.scancode == 32 { // letter d
                              println!("\n Do a barrel roll! \n"); // Star fox's Peppy line on N64
                         }
                    },
                    
                    _ => ()
               },
               
               _ => ()
          }
          
          // render
          unsafe {
               gl::ClearColor(0.2, 0.3, 0.3, 1.0);
               gl::Clear(gl::COLOR_BUFFER_BIT);
               // CLEAR DEPTH BUFFER EVERY TIME
               gl::Clear(gl::DEPTH_BUFFER_BIT);
               
               // model matrix setting
               
               // render
               for i in 0..10 {
                    if i%3 == 0 {
                         let mut model_mat: Matrix4<f32> = Matrix4::from_axis_angle(Vector3::new(0.5,1.0,0.0), Deg(time.elapsed().as_secs_f32()*50.0));
                         model_mat = translate_model_mats[i] * model_mat; 
                         shader_program.set_mat4("model", &model_mat);
                    } else {
                         let model_mat = translate_model_mats[i];
                         shader_program.set_mat4("model", &model_mat);
                    }
                    gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, ptr::null());
               }
          }
          new_context.swap_buffers().unwrap(); //needed to cahnge old and new buffer and redraw
     });
 
     // ############
     // Event Loop #
     // ############
 }
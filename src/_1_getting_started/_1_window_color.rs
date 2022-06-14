use glutin::{ContextBuilder, GlRequest, Api, GlProfile}; // Create OpenGL context and use 3.3 version
use glutin::window::WindowBuilder; // Needed to create a window
use glutin::event::{Event, WindowEvent}; // Event creation for updating view
use glutin::event_loop::{ControlFlow, EventLoop}; // Event loop
use glutin::dpi::PhysicalSize; // to specify size of window
use gl;

pub fn window_color_change() {

    let window_builder = WindowBuilder::new().with_title("Dzahui").with_inner_size(PhysicalSize {height: 500, width: 500}).with_resizable(true);    
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
    // now ussage of OpenGL functions is possible
    unsafe { gl::Viewport(0,0,500,500) }

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
                    } else if input.scancode == 48 { // letter b
                        println!("\n color being changed to blue \n");
                        unsafe {
                            gl::ClearColor(0.15,0.33,0.96,0.8); // state setting function
                            gl::Clear(gl::COLOR_BUFFER_BIT); // clearing color buffer only. state using function
                        }
                    } else if input.scancode == 21 { // letter y
                        unsafe {
                            println!("Changing color to yellow");
                            gl::ClearColor(0.90,0.92,0.0,0.8); // state setting function
                            gl::Clear(gl::COLOR_BUFFER_BIT); // clearing color buffer only. state using function
                        }
                    } else if input.scancode == 19 {
                        unsafe {
                            println!("Changing color to red");
                            gl::ClearColor(0.92,0.0,0.23,0.8); // state setting function
                            gl::Clear(gl::COLOR_BUFFER_BIT); // clearing color buffer only. state using function
                        }
                    }
                    new_context.swap_buffers().unwrap(); //needed to cahnge old and new buffer and redraw
                }
                _ => ()
            },
            Event::RedrawRequested(_) => (),
            _ => ()
        }
    })

}
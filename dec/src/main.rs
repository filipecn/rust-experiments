mod render_gl;
mod structures;
extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    // loading a specific function pointer
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));
    gl::ClearColor::load_with(|s| glfw.get_proc_address_raw(s));
    gl::Clear::load_with(|s| glfw.get_proc_address_raw(s));
    gl::ShaderSource::load_with(|s| glfw.get_proc_address_raw(s));
    gl::CompileShader::load_with(|s| glfw.get_proc_address_raw(s));
    gl::GetShaderiv::load_with(|s| glfw.get_proc_address_raw(s));
    gl::GetShaderInfoLog::load_with(|s| glfw.get_proc_address_raw(s));
    gl::CreateShader::load_with(|s| glfw.get_proc_address_raw(s));
    gl::DeleteShader::load_with(|s| glfw.get_proc_address_raw(s));
    gl::AttachShader::load_with(|s| glfw.get_proc_address_raw(s));
    gl::DetachShader::load_with(|s| glfw.get_proc_address_raw(s));
    gl::CreateProgram::load_with(|s| glfw.get_proc_address_raw(s));
    gl::GetProgramInfoLog::load_with(|s| glfw.get_proc_address_raw(s));
    gl::GetProgramiv::load_with(|s| glfw.get_proc_address_raw(s));
    gl::LinkProgram::load_with(|s| glfw.get_proc_address_raw(s));
    gl::UseProgram::load_with(|s| glfw.get_proc_address_raw(s));

    use std::ffi::CString;

    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Swap front and back buffers
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

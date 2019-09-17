extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

pub struct GraphicsDisplay {
    glfw: glfw::Glfw,
    context: (glfw::Window, Receiver<(f64, glfw::WindowEvent)>),
}

impl GraphicsDisplay {
    pub fn create(width: u32, height: u32, title: String)
    //  -> Self
    {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(width, height, "asda", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        // window.set_key_polling(true);
        // window.make_current();
        // gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));
        // gl::ClearColor::load_with(|s| glfw.get_proc_address_raw(s));
        // gl::Clear::load_with(|s| glfw.get_proc_address_raw(s));
        // Self {
        //     context :
        //     glfw: glfw,
        // }
    }
    // pub fn start(&mut self) {
    //     while !self.window.should_close() {
    //         self.glfw.poll_events();
    //         for (_, event) in glfw::flush_messages(&self.window_events) {
    //             handle_window_event(&mut self.window, event);
    //         }
    //         unsafe {
    //             gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    //             gl::Clear(gl::COLOR_BUFFER_BIT);
    //         }
    //         // Swap front and back buffers
    //         self.window.swap_buffers();
    //     }
    // }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::circe::io::GraphicsDisplay;
    #[test]
    fn initial_window() {
        //let gd =
        GraphicsDisplay::create(400, 400, "asd".to_string());
    }
}

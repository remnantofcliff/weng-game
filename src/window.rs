use anyhow::anyhow;
use glfw::Glfw;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub struct Window {
    context: Glfw,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    window: glfw::Window,
}

impl Window {
    pub fn events(&mut self) -> std::sync::mpsc::TryIter<(f64, glfw::WindowEvent)> {
        self.context.poll_events();
        self.events.try_iter()
    }
    pub fn get_relative_mouse_position(&self) -> glam::DVec2 {
        let (x, y) = self.window.get_cursor_pos();
        let (width, height) = self.window.get_framebuffer_size();

        glam::DVec2::new(x / width as f64, y / height as f64)
    }
    pub fn key_down(&self, key: glfw::Key) -> bool {
        matches!(
            self.window.get_key(key),
            glfw::Action::Press | glfw::Action::Repeat
        )
    }

    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        matches!(self.window.get_key(key), glfw::Action::Press)
    }

    pub fn new() -> anyhow::Result<Self> {
        let mut context = glfw::init::<()>(glfw::LOG_ERRORS)?;

        context.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = context
            .create_window(800, 600, "Title", glfw::WindowMode::Windowed)
            .unwrap();

        window.set_cursor_mode(glfw::CursorMode::Disabled);

        context
            .supports_raw_motion()
            .then(|| {
                window.set_raw_mouse_motion(true);

                Self {
                    context,
                    events,
                    window,
                }
            })
            .ok_or_else(|| anyhow!("Mouse raw motion unsupported"))
    }
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    pub fn get_framebuffer_size(&self) -> glam::UVec2 {
        let (width, height) = self.window.get_framebuffer_size();

        glam::UVec2::new(width as u32, height as u32)
    }
}

unsafe impl raw_window_handle::HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.window.raw_display_handle()
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl weng::WindowInfo for Window {
    fn render_size(&self) -> (u32, u32) {
        let size = self.window.get_framebuffer_size();

        (size.0 as u32, size.1 as u32)
    }
}

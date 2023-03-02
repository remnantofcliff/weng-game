use crate::window::Window;
use bitflags::bitflags;

bitflags! {
    pub struct Commands: u8 {
        const FLYING_CAMERA = 1 << 0;
        const FPS_CAMERA = 1 << 1;
    }
}
pub struct Input {
    commands: Commands,
    last_mouse_pos: glam::DVec2,
    mouse_pos: glam::DVec2,
    movement: glam::Vec3,
}

impl Input {
    pub fn flying_camera(&self) -> bool {
        self.commands.contains(Commands::FLYING_CAMERA)
    }
    pub fn fps_camera(&self) -> bool {
        self.commands.contains(Commands::FPS_CAMERA)
    }
    pub fn movement(&self) -> glam::Vec3 {
        self.movement
    }
    pub fn mouse_diff(&self) -> glam::Vec2 {
        (self.mouse_pos - self.last_mouse_pos).as_vec2()
    }
    pub fn new() -> Self {
        Self {
            commands: Commands::empty(),
            last_mouse_pos: glam::DVec2::ZERO,
            mouse_pos: glam::DVec2::ZERO,
            movement: glam::Vec3::ZERO,
        }
    }
    pub fn update(&mut self, window: &Window) {
        let move_z = window.key_down(glfw::Key::W) as i8 - window.key_down(glfw::Key::S) as i8;
        let move_x = window.key_down(glfw::Key::D) as i8 - window.key_down(glfw::Key::A) as i8;
        self.movement = glam::Vec3::new(move_x as f32, 0.0, move_z as f32).normalize_or_zero();

        self.last_mouse_pos = self.mouse_pos;
        self.mouse_pos = window.get_relative_mouse_position();
        // +y = go up
        self.mouse_pos.y = -self.mouse_pos.y;

        self.commands = [
            (glfw::Key::F8, Commands::FLYING_CAMERA),
            (glfw::Key::F9, Commands::FPS_CAMERA),
        ]
        .into_iter()
        .filter_map(|(key, command)| window.key_pressed(key).then_some(command))
        .reduce(|commands, command| commands | command)
        .unwrap_or(Commands::empty());
    }
}

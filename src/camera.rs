use crate::input::Input;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Type {
    Flying,
    Fps,
}

impl Type {
    pub fn transform_dir_for_movement(self, dir: glam::Vec3) -> glam::Vec3 {
        match self {
            Type::Flying => dir,
            Type::Fps => glam::Vec3::new(dir.x, 0.0, dir.z),
        }
    }
}

pub struct Camera {
    dir: glam::Vec3,
    aspect: f32,
    fov_y: f32,
    pos: glam::Vec3,
    type_: Type,
}

impl Camera {
    const UP: glam::Vec3 = glam::Vec3::Y;
    const Z_NEAR: f32 = 0.1;
    const Z_FAR: f32 = 100.0;

    pub fn build_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_to_lh(self.pos, self.dir, Self::UP);
        let proj = glam::Mat4::perspective_lh(self.fov_y, self.aspect, Self::Z_NEAR, Self::Z_FAR);

        proj * view
    }
    pub fn new(surface_width: u32, surface_height: u32) -> Self {
        Self {
            dir: glam::Vec3::Z,
            pos: glam::Vec3::new(0.0, 0.0, -1.5),
            aspect: surface_width as f32 / surface_height as f32,
            fov_y: f32::to_radians(50.0),
            type_: Type::Fps,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.aspect = new_width as f32 / new_height as f32;
    }

    pub fn update(&mut self, input: &Input) {
        let view_speed = 0.5;
        let mouse_diff = input.mouse_diff() * view_speed;

        let pitch = glam::Quat::from_axis_angle(glam::Vec3::X, mouse_diff.y);
        let yaw = glam::Quat::from_axis_angle(glam::Vec3::Y, mouse_diff.x);
        let orientation = pitch * yaw;

        self.dir = orientation * self.dir;

        self.dir.y = self
            .dir
            .y
            .clamp(f32::to_radians(-35.0), f32::to_radians(35.0));

        self.dir = self.dir.normalize();

        let move_speed = 0.01;
        let movement_dir = self.type_.transform_dir_for_movement(self.dir);

        self.pos += (movement_dir * input.movement().z
            + movement_dir.cross(Self::UP) * input.movement().x)
            .normalize_or_zero()
            * move_speed;

        if input.flying_camera() {
            self.type_ = Type::Flying;
        }

        if input.fps_camera() {
            self.type_ = Type::Fps;
        }
    }
}

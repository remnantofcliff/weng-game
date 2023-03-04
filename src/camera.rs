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
    yaw: f32,
    pitch: f32,
    pos: glam::Vec3,
    projection: glam::Mat4,
    type_: Type,
}

impl Camera {
    const FOVY: f32 = 65.0 * std::f32::consts::PI / 180.0;
    const UP: glam::Vec3 = glam::Vec3::Y;
    const Z_NEAR: f32 = 0.1;
    const Z_FAR: f32 = 100.0;

    pub fn build_matrix(&self) -> glam::Mat4 {
        self.projection * glam::Mat4::look_to_lh(self.pos, self.dir, Self::UP)
    }

    pub fn new(surface_width: u32, surface_height: u32) -> Self {
        let mut camera = Self {
            pos: glam::Vec3::new(0.0, 0.0, -1.5),
            projection: glam::Mat4::IDENTITY,
            type_: Type::Fps,
            yaw: 0.0,
            pitch: 0.0,
            dir: glam::Vec3::Z,
        };

        camera.resize(surface_width, surface_height);

        camera
    }

    pub fn position(&self) -> glam::Vec3 {
        self.pos
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.projection = glam::Mat4::perspective_lh(
            Self::FOVY,
            new_width as f32 / new_height as f32,
            Self::Z_NEAR,
            Self::Z_FAR,
        );
    }

    pub fn update(&mut self, input: &Input) {
        if input.flying_camera() {
            self.type_ = Type::Flying;
        }

        if input.fps_camera() {
            self.type_ = Type::Fps;
        }

        {
            let view_speed = 1.0;
            let mouse_diff = input.mouse_diff() * view_speed;
            self.yaw += mouse_diff.x;
            self.pitch += mouse_diff.y;

            self.pitch = self
                .pitch
                .clamp(f32::to_radians(-89.0), f32::to_radians(89.0));
        }

        let right = -self.dir.cross(Self::UP);

        let orientation = {
            let pitch = glam::Quat::from_axis_angle(right, -self.pitch);
            let yaw = glam::Quat::from_axis_angle(glam::Vec3::Y, self.yaw);
            pitch * yaw
        };

        self.dir = orientation.mul_vec3(glam::Vec3::Z);

        let move_speed = 0.1;

        let movement_dir = self.type_.transform_dir_for_movement(self.dir);

        self.pos += (movement_dir * input.movement().z + right * input.movement().x)
            .normalize_or_zero()
            * move_speed;
    }
}

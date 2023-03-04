pub const NAME: &str = "basic.wgsl";

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Instance {
    pub model: [f32; 16],
    pub normal: [f32; 9],
}

impl weng::graphics::instances::Instance for Instance {
    const ATTRIBUTES: &'static [weng::wgpu::VertexAttribute] = &weng::graphics::vertices::vertex_attr_array![
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
        9 => Float32x3,
        10 => Float32x3,
        11 => Float32x3,
    ];
}

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    pub pos: [f32; 4],
    pub view_proj: [f32; 16],
}

impl weng::graphics::uniforms::Uniform for CameraUniform {}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct LightUniform {
    pub position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    pub _padding: u32,
    pub color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    pub _padding2: u32,
}

impl weng::graphics::uniforms::Uniform for LightUniform {}

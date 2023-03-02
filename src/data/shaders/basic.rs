pub const NAME: &str = "basic.wgsl";

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniform {
    pub camera: [f32; 16],
}

impl weng::graphics::uniforms::Uniform for Uniform {}

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Instance {
    pub model: [f32; 16],
}

impl weng::graphics::instances::Instance for Instance {
    const ATTRIBUTES: &'static [weng::wgpu::VertexAttribute] = &weng::graphics::vertices::vertex_attr_array![
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
    ];
}

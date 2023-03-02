use image::{EncodableLayout, ImageError};
use std::path::Path;

pub static DIR: &str = "assets/textures";

pub fn load(
    graphics: &weng::graphics::Context,
    path: &Path,
    texture_bind_group_layout: &weng::wgpu::BindGroupLayout,
) -> Result<weng::wgpu::BindGroup, ImageError> {
    image::open(path).map(|image| {
        let rgba8 = image.to_rgba8();

        graphics.create_texture_bind_group(
            rgba8.as_bytes(),
            rgba8.dimensions().0,
            rgba8.dimensions().1,
            texture_bind_group_layout,
        )
    })
}

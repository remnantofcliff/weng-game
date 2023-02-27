use image::{EncodableLayout, ImageError};

pub mod grass;

pub fn load(
    graphics: &weng::graphics::Context,
    path: &str,
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

pub fn load_all<const N: usize>(
    graphics: &weng::graphics::Context,
    paths: &[&str; N],
    texture_bind_group_layout: &weng::wgpu::BindGroupLayout,
) -> Result<Vec<weng::wgpu::BindGroup>, ImageError> {
    paths
        .iter()
        .map(|path| load(graphics, path, texture_bind_group_layout))
        .collect::<_>()
}

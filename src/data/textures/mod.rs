use image::ImageError;
use std::path::Path;

pub static DIR: &str = "assets/textures";

pub trait Type {
    const FORMAT: weng::wgpu::TextureFormat;
}

pub struct Diffuse;
pub struct Normal;

impl Type for Diffuse {
    const FORMAT: weng::wgpu::TextureFormat = weng::wgpu::TextureFormat::Rgba8UnormSrgb;
}

impl Type for Normal {
    const FORMAT: weng::wgpu::TextureFormat = weng::wgpu::TextureFormat::Rgba8Unorm;
}

pub fn load<T: Type>(
    graphics: &weng::graphics::Context,
    path: &Path,
) -> Result<weng::graphics::textures::Texture, ImageError> {
    image::open(path).map(|image| {
        let rgba8 = image.to_rgba8();
        graphics.create_texture(
            &rgba8,
            rgba8.dimensions().0,
            rgba8.dimensions().1,
            T::FORMAT,
        )
    })
}

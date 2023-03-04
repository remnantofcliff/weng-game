mod camera;
mod data;
mod input;
mod time;
mod window;

use std::path::Path;

use camera::Camera;
use input::Input;
use rand::{Rng, SeedableRng};
use time::Time;
use window::Window;

fn run() -> anyhow::Result<()> {
    env_logger::init();

    let mut window = Window::new("title", 1920, 1080)?;
    let mut graphics = weng::graphics::Context::new(&window)?;

    let mut camera = Camera::new(graphics.surface_width(), graphics.surface_height());

    let shader =
        graphics.load_shader(&Path::new(data::shaders::DIR).join(data::shaders::basic::NAME))?;

    let diffuse_texture_bind_group_layout = graphics.create_texture_bind_group_layout();

    let camera_bind_group_layout =
        graphics.create_uniform_bind_group_layout(weng::wgpu::ShaderStages::VERTEX_FRAGMENT);
    let light_bind_group_layout =
        graphics.create_uniform_bind_group_layout(weng::wgpu::ShaderStages::VERTEX_FRAGMENT);

    let camera_uniform_buffer =
        graphics.create_uniform_buffer(&[data::shaders::basic::CameraUniform {
            pos: glam::Vec4::from((camera.position(), 0.0)).to_array(),
            view_proj: camera.build_matrix().to_cols_array(),
        }]);
    let light_uniform_buffer =
        graphics.create_uniform_buffer(&[data::shaders::basic::LightUniform {
            position: [0.0, 0.0, 0.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        }]);

    let camera_bind_group =
        graphics.create_uniform_bind_group(&camera_bind_group_layout, &camera_uniform_buffer);
    let light_bind_group =
        graphics.create_uniform_bind_group(&light_bind_group_layout, &light_uniform_buffer);

    let render_pipeline = graphics
        .create_pipeline_instanced::<data::models::Vertex, data::shaders::basic::Instance>(
            &shader,
            &[
                &diffuse_texture_bind_group_layout,
                &camera_bind_group_layout,
                &light_bind_group_layout,
            ],
        );

    let mut random = rand::rngs::SmallRng::from_entropy();

    let instances: [_; 100] = std::array::from_fn(|_| {
        let scale = random.gen_range(0.25..4.0);
        let rotation = random.gen();

        data::shaders::basic::Instance {
            model: glam::Mat4::from_scale_rotation_translation(
                glam::Vec3::splat(scale),
                rotation,
                glam::Vec3::new(
                    random.gen_range(-50.0..50.0),
                    random.gen_range(-10.0..10.0),
                    random.gen_range(-50.0..50.0),
                ),
            )
            .to_cols_array(),
            normal: glam::Mat3::from_quat(rotation).to_cols_array(),
        }
    });

    let instance_buffer = graphics.create_instance_buffer(&instances);

    let model = data::models::Model::load(&mut graphics, &diffuse_texture_bind_group_layout)?;

    let mut input = Input::new();
    let mut time = Time::new();

    let mut fb_size = window.get_framebuffer_size();

    while !window.should_close() {
        time.begin_loop();

        let new_fb_size = window.get_framebuffer_size();
        if fb_size != new_fb_size {
            resize(&mut graphics, &mut camera, new_fb_size);
        }
        fb_size = new_fb_size;
        window.events();

        while time.should_update() {
            input.update(&window);
            camera.update(&input);
            time.update();
        }

        camera_uniform_buffer.set(
            &graphics,
            &[data::shaders::basic::CameraUniform {
                pos: glam::Vec4::from((camera.position(), 0.0)).to_array(),
                view_proj: camera.build_matrix().to_cols_array(),
            }],
        );

        let render_results = [graphics.render(
            &render_pipeline,
            &model.meshes[0].vertex_buffer,
            &model.meshes[0].index_buffer,
            &instance_buffer,
            [
                &model.materials[model.meshes[0].material_indice].texture_bind_group,
                &camera_bind_group,
                &light_bind_group,
            ]
            .into_iter(),
        )];
        for result in render_results {
            match result {
                Ok(_) => (),
                Err(weng::wgpu::SurfaceError::Lost | weng::wgpu::SurfaceError::Outdated) => {
                    let size = window.get_framebuffer_size();

                    resize(&mut graphics, &mut camera, size);
                }
                Err(weng::wgpu::SurfaceError::OutOfMemory) => {
                    log::error!("out of memory, exiting");

                    return Ok(());
                }
                Err(weng::wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}

fn resize(
    graphics: &mut weng::graphics::Context,
    camera: &mut Camera,
    framebuffer_size: glam::UVec2,
) {
    graphics.resize(framebuffer_size.x, framebuffer_size.y);
    camera.resize(framebuffer_size.x, framebuffer_size.y);
}

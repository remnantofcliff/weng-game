mod camera;
mod data;
mod input;
mod time;
mod window;

use std::path::Path;

use camera::Camera;
use input::Input;
use lazy_static::lazy_static;
use time::Time;
use window::Window;

lazy_static! {
    pub static ref INSTANCES: [data::shaders::basic::Instance; 4] = [
        data::shaders::basic::Instance {
            model: glam::Mat4::from_rotation_translation(
                glam::Quat::from_rotation_y(f32::to_radians(10.0)),
                glam::vec3(1.0, 0.0, 0.0)
            )
            .to_cols_array(),
        },
        data::shaders::basic::Instance {
            model: glam::Mat4::from_rotation_translation(
                glam::Quat::from_rotation_y(f32::to_radians(15.0)),
                glam::vec3(10.0, 0.0, 0.0)
            )
            .to_cols_array(),
        },
        data::shaders::basic::Instance {
            model: glam::Mat4::from_rotation_translation(
                glam::Quat::from_rotation_y(f32::to_radians(30.0)),
                glam::vec3(100.0, 0.0, 0.0)
            )
            .to_cols_array(),
        },
        data::shaders::basic::Instance {
            model: glam::Mat4::from_translation(glam::vec3(4.0, 0.0, 0.0)).to_cols_array(),
        },
    ];
}

fn run() -> anyhow::Result<()> {
    env_logger::init();

    let mut window = Window::new("title", 1920, 1080)?;
    let mut graphics = weng::graphics::Context::new(&window)?;

    let mut camera = Camera::new(graphics.surface_width(), graphics.surface_height());

    let shader =
        graphics.load_shader(&Path::new(data::shaders::DIR).join(data::shaders::basic::NAME))?;
    let diffuse_texture_bind_group_layout = graphics.create_texture_bind_group_layout();
    let uniform_bind_group_layout = graphics.create_uniform_bind_group_layout();

    let render_pipeline = graphics
        .create_pipeline::<data::models::Vertex, data::shaders::basic::Instance>(
            &shader,
            &[
                &diffuse_texture_bind_group_layout,
                &uniform_bind_group_layout,
            ],
        );

    let uniform_buffer = graphics.create_uniform_buffer(&[data::shaders::basic::Uniform {
        camera: camera.build_matrix().to_cols_array(),
    }]);
    let instance_buffer = graphics.create_instance_buffer(INSTANCES.as_slice());

    let camera_uniform_bind_group =
        graphics.create_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer);

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

        uniform_buffer.set(
            &graphics,
            &[data::shaders::basic::Uniform {
                camera: camera.build_matrix().to_cols_array(),
            }],
        );

        match graphics.render(
            &render_pipeline,
            &model.vertex_buffers[0],
            &model.index_buffers[0],
            &instance_buffer,
            [
                &model.bind_groups[model.material_indices[0]],
                &camera_uniform_bind_group,
            ]
            .into_iter(),
        ) {
            Ok(_) => (),
            Err(weng::wgpu::SurfaceError::Lost | weng::wgpu::SurfaceError::Outdated) => {
                let size = window.get_framebuffer_size();

                resize(&mut graphics, &mut camera, size);
            }
            Err(weng::wgpu::SurfaceError::OutOfMemory) => {
                log::error!("out of memory, exiting");

                break;
            }
            Err(weng::wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
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

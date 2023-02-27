mod camera;
mod data;
mod input;
mod time;
mod window;

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
                glam::vec3(2.0, 0.0, 0.0)
            )
            .to_cols_array(),
        },
        data::shaders::basic::Instance {
            model: glam::Mat4::from_rotation_translation(
                glam::Quat::from_rotation_y(f32::to_radians(30.0)),
                glam::vec3(3.0, 0.0, 0.0)
            )
            .to_cols_array(),
        },
        data::shaders::basic::Instance {
            model: glam::Mat4::from_translation(glam::vec3(4.0, 0.0, 0.0)).to_cols_array(),
        },
    ];
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut window = Window::new()?;
    let mut graphics = weng::graphics::Context::new(&window)?;

    let mut camera = Camera::new(graphics.surface_width(), graphics.surface_height());

    let shader = graphics.load_shader(data::shaders::basic::PATH)?;

    let texture_bind_group_layout = graphics.create_texture_bind_group_layout();
    let uniform_bind_group_layout = graphics.create_uniform_bind_group_layout();

    let render_pipeline = graphics
        .create_pipeline::<data::models::Vertex, data::shaders::basic::Instance>(
            &shader,
            &[&texture_bind_group_layout, &uniform_bind_group_layout],
        );

    let uniform_buffer = graphics.create_uniform_buffer(&[data::shaders::basic::Uniform {
        camera: camera.build_matrix().to_cols_array(),
    }]);
    let instance_buffer = graphics.create_instance_buffer(INSTANCES.as_slice());

    let mut current_texture_bind_group = 0;
    let texture_bind_groups = data::textures::load_all(
        &graphics,
        &data::textures::grass::PATHS,
        &texture_bind_group_layout,
    )?;

    let camera_uniform_bind_group =
        graphics.create_uniform_bind_group(&uniform_bind_group_layout, &uniform_buffer);

    let model = data::models::Model::load(&mut graphics, &texture_bind_group_layout)?;

    let mut input = Input::new();
    let mut time = Time::new();

    while !window.should_close() {
        time.begin_loop();

        for (_, event) in window.events() {
            if let glfw::WindowEvent::FramebufferSize(width, height) = event {
                resize(
                    &mut graphics,
                    &mut camera,
                    glam::UVec2::new(width as u32, height as u32),
                );
            }
        }

        while time.should_update() {
            input.update(&window);
            input.change_texture().then(|| {
                current_texture_bind_group =
                    (current_texture_bind_group + 1) % texture_bind_groups.len()
            });
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
            &model
                .meshes
                .iter()
                .map(|mesh| mesh.vertex_buffer.slice(..))
                .chain(std::iter::once(instance_buffer.slice(..)))
                .collect::<Vec<_>>(),
            (
                index_buffer.slice(..),
                data::models::pentagon::INDICES.len() as u32,
            ),
            (instance_buffer.slice(..), INSTANCES.len() as u32),
            &[
                &texture_bind_groups[current_texture_bind_group],
                &camera_uniform_bind_group,
            ],
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

fn resize(
    graphics: &mut weng::graphics::Context,
    camera: &mut Camera,
    framebuffer_size: glam::UVec2,
) {
    graphics.resize_surface(framebuffer_size.x, framebuffer_size.y);
    camera.resize(framebuffer_size.x, framebuffer_size.y);
}

pub static DIR: &str = "assets/models";

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl weng::graphics::vertices::Vertex for Vertex {
    const ATTRIBUTES: &'static [weng::wgpu::VertexAttribute] = &weng::graphics::vertices::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x3,
    ];
}

pub struct Model<I: weng::graphics::indices::IndexType, V: weng::graphics::vertices::Vertex> {
    pub bind_groups: Vec<weng::wgpu::BindGroup>,
    pub index_buffers: Vec<weng::graphics::indices::IndexBuffer<I>>,
    pub vertex_buffers: Vec<weng::graphics::vertices::VertexBuffer<V>>,
    pub material_indices: Vec<usize>,
}

impl Model<u32, Vertex> {
    pub fn load(
        graphics: &mut weng::graphics::Context,
        texture_bind_group_layout: &weng::wgpu::BindGroupLayout,
    ) -> anyhow::Result<Self> {
        let (models, material_infos) = tobj::load_obj_buf(
            &mut std::io::BufReader::new(std::fs::File::open("assets/models/cube.obj")?),
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
            |p| {
                tobj::load_mtl_buf(&mut std::io::BufReader::new(
                    std::fs::File::open(std::path::Path::new(DIR).join(p)).unwrap(),
                ))
            },
        )?;
        let bind_groups = material_infos?
            .into_iter()
            .map(|material| {
                crate::data::textures::load(
                    graphics,
                    &std::path::Path::new(crate::data::textures::DIR)
                        .join(material.diffuse_texture),
                    texture_bind_group_layout,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut vertex_buffers = Vec::with_capacity(models.len());
        let mut index_buffers = Vec::with_capacity(models.len());
        let mut material_indices = Vec::with_capacity(models.len());

        for model in models {
            let vertices = (0..model.mesh.positions.len() / 3)
                .map(|i| crate::data::models::Vertex {
                    position: [
                        model.mesh.positions[i * 3],
                        model.mesh.positions[i * 3 + 1],
                        model.mesh.positions[i * 3 + 2],
                    ],
                    texture_coords: [model.mesh.texcoords[i * 2], model.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        model.mesh.normals[i * 3],
                        model.mesh.normals[i * 3 + 1],
                        model.mesh.normals[i * 3 + 2],
                    ],
                })
                .collect::<Vec<_>>();

            vertex_buffers.push(graphics.create_vertex_buffer(&vertices));
            index_buffers.push(graphics.create_index_buffer(&model.mesh.indices));
            material_indices.push(model.mesh.material_id.unwrap_or(0));
        }

        Ok(Model {
            bind_groups,
            index_buffers,
            vertex_buffers,
            material_indices,
        })
    }
}

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

pub struct Mesh<I: weng::graphics::indices::IndexType> {
    pub vertex_buffer: weng::wgpu::Buffer,
    pub index_buffer: weng::graphics::indices::IndexBuffer<I>,
    pub num_elements: u32,
    pub material: usize,
}

pub struct Material {
    pub bind_group: weng::wgpu::BindGroup,
}

pub struct Model<I: weng::graphics::indices::IndexType> {
    pub meshes: Vec<Mesh<I>>,
    pub materials: Vec<Material>,
}

impl Model<u32> {
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
                    std::fs::File::open(p).unwrap(),
                ))
            },
        )?;
        let materials = material_infos?
            .into_iter()
            .map(|material| {
                crate::data::textures::load(
                    graphics,
                    &material.diffuse_texture,
                    texture_bind_group_layout,
                )
                .map(|bind_group| Material { bind_group })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let meshes = models
            .into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| crate::data::models::Vertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        texture_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                    })
                    .collect::<Vec<_>>();

                let vertex_buffer = graphics.create_vertex_buffer(vertices.as_slice());
                let index_buffer = graphics.create_index_buffer(m.mesh.indices.as_slice());

                Mesh {
                    vertex_buffer,
                    index_buffer,
                    num_elements: m.mesh.indices.len() as u32,
                    material: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

        Ok(Model { meshes, materials })
    }
}

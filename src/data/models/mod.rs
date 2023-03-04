use std::path::Path;

pub static DIR: &str = "assets/models";

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}

impl weng::graphics::vertices::Vertex for Vertex {
    const ATTRIBUTES: &'static [weng::wgpu::VertexAttribute] = &weng::graphics::vertices::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
    ];
}

pub struct Material {
    pub texture_bind_group: weng::wgpu::BindGroup,
}

pub struct Mesh<V: weng::graphics::vertices::Vertex> {
    pub index_buffer: weng::graphics::indices::IndexBuffer<u32>,
    pub material_indice: usize,
    pub vertex_buffer: weng::graphics::vertices::VertexBuffer<V>,
}

pub struct Model<V: weng::graphics::vertices::Vertex> {
    pub meshes: Vec<Mesh<V>>,
    pub materials: Vec<Material>,
}

impl Model<Vertex> {
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
                    std::fs::File::open(Path::new(DIR).join(p)).unwrap(),
                ))
            },
        )?;

        let material_infos = material_infos?;

        let mut materials = Vec::with_capacity(material_infos.len());

        for info in material_infos {
            let diffuse_texture = crate::data::textures::load::<crate::data::textures::Diffuse>(
                graphics,
                &Path::new(crate::data::textures::DIR).join(info.diffuse_texture),
            )?;
            let normal_texture = crate::data::textures::load::<crate::data::textures::Normal>(
                graphics,
                &Path::new(crate::data::textures::DIR).join(info.normal_texture),
            )?;

            let bind_group = graphics.create_texture_bind_group(
                &diffuse_texture,
                &normal_texture,
                texture_bind_group_layout,
            );

            materials.push(Material {
                texture_bind_group: bind_group,
            });
        }

        let mut meshes = Vec::with_capacity(models.len());

        for model in models {
            let mut vertices = (0..model.mesh.positions.len() / 3)
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
                    tangent: [0.0; 3],
                    bitangent: [0.0; 3],
                })
                .collect::<Vec<_>>();

            let indices = &model.mesh.indices;
            let mut triangles_included = vec![0; vertices.len()];

            for c in indices.chunks(3) {
                let v0 = vertices[c[0] as usize];
                let v1 = vertices[c[1] as usize];
                let v2 = vertices[c[2] as usize];

                let pos0 = glam::Vec3::from_array(v0.position);
                let pos1 = glam::Vec3::from_array(v1.position);
                let pos2 = glam::Vec3::from_array(v2.position);

                let uv0 = glam::Vec2::from_array(v0.texture_coords);
                let uv1 = glam::Vec2::from_array(v1.texture_coords);
                let uv2 = glam::Vec2::from_array(v2.texture_coords);

                // Calculate the edges of the triangle
                let delta_pos1 = pos1 - pos0;
                let delta_pos2 = pos2 - pos0;

                // This will give us a direction to calculate the
                // tangent and bitangent
                let delta_uv1 = uv1 - uv0;
                let delta_uv2 = uv2 - uv0;

                // Solving the following system of equations will
                // give us the tangent and bitangent.
                //     delta_pos1 = delta_uv1.x * T + delta_u.y * B
                //     delta_pos2 = delta_uv2.x * T + delta_uv2.y * B
                // Luckily, the place I found this equation provided
                // the solution!
                let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                // We flip the bitangent to enable right-handed normal
                // maps with wgpu texture coordinate system
                let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

                // We'll use the same tangent/bitangent for each vertex in the triangle
                vertices[c[0] as usize].tangent =
                    (tangent + glam::Vec3::from_array(vertices[c[0] as usize].tangent)).to_array();
                vertices[c[1] as usize].tangent =
                    (tangent + glam::Vec3::from_array(vertices[c[1] as usize].tangent)).to_array();
                vertices[c[2] as usize].tangent =
                    (tangent + glam::Vec3::from_array(vertices[c[2] as usize].tangent)).to_array();
                vertices[c[0] as usize].bitangent = (bitangent
                    + glam::Vec3::from_array(vertices[c[0] as usize].bitangent))
                .to_array();
                vertices[c[1] as usize].bitangent = (bitangent
                    + glam::Vec3::from_array(vertices[c[1] as usize].bitangent))
                .to_array();
                vertices[c[2] as usize].bitangent = (bitangent
                    + glam::Vec3::from_array(vertices[c[2] as usize].bitangent))
                .to_array();

                // Used to average the tangents/bitangents
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }

            // Average the tangents/bitangents
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denom = 1.0 / n as f32;
                let mut v = &mut vertices[i];
                v.tangent = (glam::Vec3::from_array(v.tangent) * denom).to_array();
                v.bitangent = (glam::Vec3::from_array(v.bitangent) * denom).to_array();
            }

            meshes.push(Mesh {
                index_buffer: graphics.create_index_buffer(&model.mesh.indices),
                material_indice: model.mesh.material_id.unwrap_or(0),
                vertex_buffer: graphics.create_vertex_buffer(&vertices),
            });
        }

        Ok(Model { meshes, materials })
    }
}

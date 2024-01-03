use gltf::Error;

use crate::vertex::Vertex;

pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub fn load(path: &str) -> Result<MeshData, Error> {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];

    let (gltf, buffers, _) = gltf::import(path)?;
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            if let Some(iter) = reader.read_positions() {
                for position in iter {
                    vertices.push(Vertex {
                        position,
                        color: [1.0, 1.0, 1.0],
                        tex_coords: [0.0, 0.0],
                    })
                }
            }

            if let Some(iter) = reader.read_indices() {
                for index in iter.into_u32() {
                    indices.push(index);
                }
            }
        }
    }

    Ok(MeshData { vertices, indices })
}

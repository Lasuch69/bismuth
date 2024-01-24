use gltf::Error;

use crate::rendering::vertex::Vertex;

pub fn load(path: &str) -> Result<(Vec<Vertex>, Vec<u32>), Error> {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![];

    let (gltf, buffers, _) = gltf::import(path)?;
    for mesh in gltf.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut iter_position = reader.read_positions().unwrap();
            let mut iter_color = reader.read_colors(0).unwrap().into_rgb_f32();
            let mut iter_uv = reader.read_tex_coords(0).unwrap().into_f32();

            loop {
                let mut vertex = Vertex::default();

                if let Some(position) = iter_position.next() {
                    vertex.position = position;
                } else {
                    break;
                }

                if let Some(color) = iter_color.next() {
                    vertex.color = color;
                }

                if let Some(uv) = iter_uv.next() {
                    vertex.tex_coords = uv;
                }

                vertices.push(vertex);
            }

            if let Some(iter) = reader.read_indices() {
                for index in iter.into_u32() {
                    indices.push(index);
                }
            }
        }
    }

    Ok((vertices, indices))
}

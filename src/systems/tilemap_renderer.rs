use crate::components::Tilemap;
use bevy::{math::vec2, prelude::*, render::mesh::Indices};

pub fn tilemap_renderer(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Tilemap, &mut Handle<Mesh>)>,
) {
    query.for_each_mut(|(grid, mesh)| {
        let mut m = meshes
            .get_mut(mesh.id)
            .expect("mesh was not found for grid");

        let scale = 32.0;
        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        let mut indicies: Vec<u32> = Vec::new();
        let size = grid.size;
        let mut i = 0;
        for y in 0..size {
            for x in 0..size {
                let index = y * size + x;
                let cell = grid.cells.get(index).expect("grid was out of bounds");

                let north_west = vec2(x as f32 * scale, y as f32 * scale + scale);
                let north_east = vec2(x as f32 * scale + scale, y as f32 * scale + scale);
                let south_west = vec2(x as f32 * scale, y as f32 * scale);
                let south_east = vec2(x as f32 * scale + scale, y as f32 * scale);

                let tex_w = 1.0 / grid.sheet_width as f32;
                let tex_h = 1.0 / grid.sheet_height as f32;
                let x = (cell.index % grid.sheet_width) as f32;
                let y = (cell.index / grid.sheet_height) as f32;
                let u = x * tex_w;
                let v = y * tex_h;

                let vertices = [
                    (
                        [south_west.x, south_west.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u, v + tex_h],
                    ),
                    ([north_west.x, north_west.y, 0.0], [0.0, 0.0, 1.0], [u, v]),
                    (
                        [north_east.x, north_east.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + tex_w, v],
                    ),
                    (
                        [south_east.x, south_east.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + tex_w, v + tex_h],
                    ),
                ];

                for (position, normal, uv) in vertices.iter() {
                    positions.push(*position);
                    normals.push(*normal);
                    uvs.push(*uv);
                }

                indicies.push(i + 0);
                indicies.push(i + 2);
                indicies.push(i + 1);
                indicies.push(i + 0);
                indicies.push(i + 3);
                indicies.push(i + 2);

                i += 4;
            }
        }

        m.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        m.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        m.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        m.set_indices(Some(Indices::U32(indicies)));
    });
}

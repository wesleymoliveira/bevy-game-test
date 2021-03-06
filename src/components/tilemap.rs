use bevy::{
    math::vec2,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Tile {
    pub index: u32,
}

impl Default for Tile {
    fn default() -> Self {
        Tile { index: 0 }
    }
}

#[derive(Component)]
pub struct Tilemap {
    pub size: usize,
    pub cells: Vec<Tile>,
    pub sheet_width: u32,
    pub sheet_height: u32,
}

impl Tilemap {
    pub fn new(size: usize, sheet_width: u32, sheet_height: u32) -> Tilemap {
        let mut grid = Tilemap {
            size,
            cells: vec![Tile::default(); size * size],
            sheet_width,
            sheet_height,
        };
        return grid;
    }

    pub fn set_tile(&mut self, tile: Tile, x: usize, y: usize) {
        self.cells[y * self.size + x] = tile;
    }

    pub fn randomize(&mut self) {
        let max = self.sheet_height * self.sheet_width;
        let mut rng = rand::thread_rng();
        for cell in &mut self.cells {
            let index: u32 = rng.gen();
            cell.index = index % max;
        }
    }

    pub fn insert_entity(
        tilemap: Tilemap,
        texture_path: &str,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Entity {
        let texture_handle: Handle<Image> = asset_server.load(texture_path);
        let mut mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let positions = Vec::<[f32; 3]>::new();
        let normals = Vec::<[f32; 3]>::new();
        let uvs = Vec::<[f32; 2]>::new();
        let indicies: Vec<u32> = Vec::new();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh.set_indices(Some(Indices::U32(indicies)));

        let mesh = meshes.add(mesh);

        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            unlit: true,
            ..Default::default()
        });

        let mut entity_commands = commands.spawn();
        entity_commands.insert(tilemap);
        entity_commands.insert_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material_handle,
            ..Default::default()
        });

        entity_commands.id()
    }
}

pub fn tilemap_renderer(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Tilemap, &mut Handle<Mesh>)>,
) {
    query.for_each_mut(|(grid, mesh)| {
        let mut m = meshes
            .get_mut(mesh.id)
            .expect("mesh was not found for grid");
        let scale = 1.0;
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

                // alpha is used to shift the texture samples 'a bit inwards' to avoid artifacts when rendering
                // different resolutions now power of 2
                let alpha = 0.001;
                let u = x * tex_w;
                let v = y * tex_h;

                let vertices = [
                    (
                        [south_west.x, south_west.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + alpha, v + tex_h - alpha],
                    ),
                    (
                        [north_west.x, north_west.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + alpha, v + alpha],
                    ),
                    (
                        [north_east.x, north_east.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + tex_w - alpha, v + alpha],
                    ),
                    (
                        [south_east.x, south_east.y, 0.0],
                        [0.0, 0.0, 1.0],
                        [u + tex_w - alpha, v + tex_h - alpha],
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

        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        m.set_indices(Some(Indices::U32(indicies)));
    });
}

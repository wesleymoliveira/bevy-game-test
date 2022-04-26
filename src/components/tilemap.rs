use bevy::{
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

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

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

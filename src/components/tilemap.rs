use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Cell {
    pub index: u32,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { index: 0 }
    }
}

#[derive(Component)]
pub struct Tilemap {
    pub size: usize,
    pub cells: Vec<Cell>,
    pub sheet_width: u32,
    pub sheet_height: u32,
}

impl Tilemap {
    pub fn new(size: usize, sheet_width: u32, sheet_height: u32) -> Tilemap {
        let mut grid = Tilemap {
            size,
            cells: vec![Cell::default(); size * size],
            sheet_width,
            sheet_height,
        };
        grid
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
        mut commands: &mut Commands,
        asset_server: Res<AssetServer>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> Entity {
        let size = 16;

        let texture_handle: Handle<Image> = asset_server.load(texture_path);
        let mut m: Mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let positions = Vec::<[f32; 3]>::new();
        let normals = Vec::<[f32; 3]>::new();
        let uvs = Vec::<[f32; 2]>::new();
        let indicies: Vec<u32> = Vec::new();

        m.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        m.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        m.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        m.set_indices(Some(Indices::U32(indicies)));

        let m = meshes.add(m);

        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            unlit: true,
            ..Default::default()
        });

        let mut e = commands.spawn();
        e.insert(tilemap);
        e.insert_bundle(PbrBundle {
            mesh: m.clone(),
            material: material_handle,
            ..Default::default()
        });

        e.id()
    }
}

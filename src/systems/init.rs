use crate::entities::{InitialState, Ufo};
use bevy::{
    math::vec2,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    println!("initializing game with an sprite");
    let texture_handle = asset_server.load("icon.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle((
            InitialState {
                in_progress: false,
                timer: 0.0,
            },
            Ufo { is_alive: true },
        ));

    let l = 32;
    /* for y in 0..l {
        for x in 0..l {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (x as f32) * 16.0,
                    (y as f32) * 16.0,
                    0.0,
                )),
                ..Default::default()
            });
        }
    } */

    let mut m: Mesh = Mesh::new(PrimitiveTopology::TriangleList); //shape::Quad::new(quad_width);

    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();
    let mut indicies: Vec<u32> = Vec::new();
    let size = 64;

    let scale = 32.0;
    let mut i = 0;
    for y in 0..size {
        for x in 0..size {
            let north_west = vec2(x as f32 * scale, y as f32 * scale + scale);
            let north_east = vec2(x as f32 * scale + scale, y as f32 * scale + scale);
            let south_west = vec2(x as f32 * scale, y as f32 * scale);
            let south_east = vec2(x as f32 * scale + scale, y as f32 * scale);

            let vertices = [
                (
                    [south_west.x, south_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 1.0],
                ),
                (
                    [north_west.x, north_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                ),
                (
                    [north_east.x, north_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0],
                ),
                (
                    [south_east.x, south_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 1.0],
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

    let m = meshes.add(m);

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        unlit: true,
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: m.clone(),
        material: material_handle,
        ..Default::default()
    });
}

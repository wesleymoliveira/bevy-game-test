use crate::components::{Thrust, Tile, Tilemap};

use bevy::prelude::*;
pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    println!("initializing game by spawning non optional entities");
    init_camera(&mut commands);
    init_map(&mut commands, &asset_server, &mut materials, &mut meshes);
    init_player(
        &mut commands,
        &asset_server,
        &mut materials,
        &mut texture_atlases,
    );
}

fn init_player(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tanks.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(Thrust { x: 1.0, y: 0.0 });
}

fn init_map(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
) {
    let size = 8;
    let mut tilemap = Tilemap::new(size, 4, 4);
    for y in 0..size {
        tilemap.set_tile(Tile { index: 1 }, 0, y);
        tilemap.set_tile(Tile { index: 1 }, size - 1, y);
    }

    for x in 0..size {
        tilemap.set_tile(Tile { index: 1 }, x, 0);
        tilemap.set_tile(Tile { index: 1 }, x, size - 1);
    }
    Tilemap::insert_entity(
        tilemap,
        "tiles.png",
        commands,
        &asset_server,
        &mut materials,
        &mut meshes,
    );
}

fn init_camera(mut commands: &mut Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

use crate::components::Tilemap;

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
    init_grid(&mut commands, asset_server, materials, meshes);
}

fn init_grid(
    mut commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut grid = Tilemap::new(8, 4, 4);

    grid.randomize();

    Tilemap::insert_entity(
        grid,
        "spritesheet3.png",
        commands,
        asset_server,
        materials,
        meshes,
    );
}

fn init_camera(mut commands: &mut Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

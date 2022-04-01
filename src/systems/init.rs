use crate::entities::{InitialState, Ufo};
use bevy::prelude::*;

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("initializing game with an sprite");
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 16);
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
    for y in 0..l {
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
    }
}

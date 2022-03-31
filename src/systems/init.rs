use crate::entities::{InitialState, Ufo};
use bevy::prelude::*;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("initializing game with an sprite");

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("ufo.png"),
            ..Default::default()
        })
        .insert_bundle((
            InitialState {
                in_progress: false,
                timer: 0.0,
            },
            Ufo { is_alive: true },
        ));
}

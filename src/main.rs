use bevy::prelude::*;

mod components;

mod entities;

mod systems;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init)
        .add_system(tilemap_renderer)
        .add_system(game)
        .run();
}

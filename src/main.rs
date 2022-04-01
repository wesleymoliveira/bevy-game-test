use bevy::prelude::*;

mod components;

mod entities;

mod systems;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_system.system())
        .add_system(tilemap_renderer.system())
        .add_system(movement_system.system())
        .run();
}

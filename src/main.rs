use bevy::prelude::*;

mod components;
pub use components::*;

mod events;
pub use events::*;

mod entities;
pub use entities::*;

mod systems;
use systems::*;

fn startup_system(mut commands: Commands, mut new_game_writer: EventWriter<NewGameEvent>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    new_game_writer.send(NewGameEvent::default());
}

// https://github.com/bevyengine/bevy/tree/v0.5.0/examples/2d
fn main() {
    let mut builder = App::new();
    // add plugins
    builder.add_plugins(DefaultPlugins);

    // add events
    builder.add_event::<NewGameEvent>();

    // add systems
    builder
        .add_startup_system(startup_system.system())
        .add_system(input_system.system())
        .add_system(game_system.system())
        .add_system(tilemap_renderer.system())
        .add_system(movement_system.system())
        .add_system(camera_system.system());

    builder.run();
}

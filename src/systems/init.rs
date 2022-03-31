use crate::entities::State;
use bevy::prelude::*;

pub fn init(mut commands: Commands) {
    println!("initializing game by spawning a state entity");
    commands.spawn().insert(State {
        in_progress: false,
        timer: 0.0,
    });
}

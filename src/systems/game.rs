use crate::entities::InitialState;
use bevy::prelude::*;

pub fn game(mut query: Query<&mut InitialState>, time: Res<Time>) {
    let state = &mut query.iter_mut().next().unwrap();

    state.timer += time.delta_seconds();

    if !state.in_progress && state.timer > 5.0 {
        println!("Five seconds passed");
        state.timer = 0.0;
    }
}

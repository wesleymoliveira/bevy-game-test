use crate::entities::{InitialState, Ufo};
//use bevy::ecs::Query;
use bevy::prelude::*;

pub fn game(
    mut state_query: Query<&mut InitialState>,
    time: Res<Time>,
    mut ufo_query: Query<&mut Ufo>,
) {
    /* let state = &mut query.iter_mut().next().unwrap();

    state.timer += time.delta_seconds();

    if !state.in_progress && state.timer > 5.0 {
        println!("Five seconds passed");
        state.timer = 0.0;
    } */
    println!("{}", time.delta_seconds() * 1000.0);
}

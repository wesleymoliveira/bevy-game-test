use bevy::prelude::*;

use crate::components::Thrust;

pub fn movement_system(mut query: Query<(&mut Thrust, &mut Transform)>, time: Res<Time>) {
    query.for_each_mut(|(thrust, mut transform)| {
        transform.translation.x += thrust.force.x * time.delta_seconds();
        transform.translation.y += thrust.force.y * time.delta_seconds();
    });
}

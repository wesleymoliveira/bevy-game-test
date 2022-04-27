use crate::{NewGameEvent, Thrust, Ufo};
use bevy::prelude::*;

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut new_game: EventWriter<NewGameEvent>,
    mut query: Query<(&Ufo, &mut Thrust)>,
) {
    if keyboard_input.just_pressed(KeyCode::F5) {
        new_game.send(NewGameEvent::default());
    }

    if let Ok((_ufo, mut thrust)) = query.get_single_mut() {
        let speed = 2.0;
        let mut v = Vec3::default();
        if keyboard_input.pressed(KeyCode::W) {
            v.y = 1.0;
        } else if keyboard_input.pressed(KeyCode::S) {
            v.y = -1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            v.x = -1.0;
        } else if keyboard_input.pressed(KeyCode::D) {
            v.x = 1.0;
        }

        if v.length() > 0.0 {
            v = v.normalize() * speed;
        }

        thrust.force = v;
    }
}

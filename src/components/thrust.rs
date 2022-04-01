use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Thrust {
    pub x: f32,
    pub y: f32,
}

use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Thrust {
    pub force: Vec2,
}

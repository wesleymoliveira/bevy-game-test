use bevy::{math::Vec3, prelude::*};

#[derive(Component, Default, Copy, Clone)]
pub struct Turret {
    pub target: Vec3,
}

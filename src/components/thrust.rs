use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Thrust {
    pub force: Vec3,
    pub constrained: bool,
}

impl Default for Thrust {
    fn default() -> Self {
        Self {
            force: Vec3::new(0.0, 0.0, 0.0),
            constrained: true,
        }
    }
}

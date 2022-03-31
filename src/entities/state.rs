use bevy::prelude::Component;

#[derive(Component)]
pub struct State {
    pub in_progress: bool,
    pub timer: f32,
}

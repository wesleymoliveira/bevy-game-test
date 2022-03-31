use bevy::prelude::Component;

#[derive(Component)]
pub struct InitialState {
    pub in_progress: bool,
    pub timer: f32,
}

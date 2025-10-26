use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Lifetime {
    pub seconds_left: f32,
}

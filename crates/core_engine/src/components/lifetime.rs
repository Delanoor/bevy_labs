use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone, Reflect)]
pub struct Lifetime {
    pub seconds_left: f32,
}

impl Lifetime {
    pub fn seconds(seconds: f32) -> Self {
        Self {
            seconds_left: seconds.max(0.0),
        }
    }
}

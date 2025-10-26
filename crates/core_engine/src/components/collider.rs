use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

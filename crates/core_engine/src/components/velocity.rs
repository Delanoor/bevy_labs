use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub lin_vel: Vec2,
    pub drag: f32, // 0..1 fraction per sec, 0 -> no drag, 1 -> instant stop like your brain
}

impl Velocity {
    pub fn new(v: Vec2) -> Self {
        Self {
            lin_vel: v,
            drag: 0.0,
        }
    }

    pub fn with_drag(v: Vec2, drag: f32) -> Self {
        Self { lin_vel: v, drag }
    }
}

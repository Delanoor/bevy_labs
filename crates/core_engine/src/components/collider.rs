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

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ColliderDebug {
    pub enabled: bool,
    pub radius_scale: f32,
    pub line_thickness: f32,
}

impl Default for ColliderDebug {
    fn default() -> Self {
        Self {
            enabled: true,
            radius_scale: 1.0,
            line_thickness: 2.0,
        }
    }
}

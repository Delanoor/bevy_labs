use bevy::asset::Asset;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, Deserialize, Reflect, Resource, Clone)]
#[reflect(Resource)]
pub struct PlayerConfig {
    pub collider_radius: f32,
    pub sprite_w: f32,
    pub sprite_h: f32,
    pub speed: f32,
}

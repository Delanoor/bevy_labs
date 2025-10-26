use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct HealEvent {
    pub target: Entity,
    pub amount: f32,
}

#[derive(Message, Component, Debug, Clone, Copy)]
pub struct DeathEvent {
    pub entity: Entity,
}

// TODO add SpawnEvent, DeathEvent, PickupEvent, etc

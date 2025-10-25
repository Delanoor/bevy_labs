use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub struct DamageEvent {
    pub entity: Entity,
    pub amount: f32,
}

// TODO add SpawnEvent, DeathEvent, PickupEvent, etc

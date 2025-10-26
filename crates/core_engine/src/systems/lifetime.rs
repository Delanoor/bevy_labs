use crate::{components::Lifetime, events::DeathEvent};
use bevy::prelude::*;

// counts down lifetimes and despawn when done
pub fn tick_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut Lifetime)>,
) {
    let dt = time.delta_secs();
    for (e, mut life) in q.iter_mut() {
        life.seconds_left -= dt;
        if life.seconds_left <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

/// optional helper to despawn entities that died (listens to DeathEvent).
pub fn despawn_on_death(mut commands: Commands, mut reader: MessageReader<DeathEvent>) {
    for ev in reader.read() {
        commands.entity(ev.entity).despawn();
    }
}

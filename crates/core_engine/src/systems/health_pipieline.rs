use crate::{components::Health, events::*};
use bevy::prelude::*;

// ticks invulnerability frames
pub fn tick_health(mut q: Query<&mut Health>, time: Res<Time>) {
    let dt = time.delta_secs();
    for mut h in q.iter_mut() {
        if h.i_frames > 0.0 {
            h.i_frames = (h.i_frames - dt).max(0.0);
        }

        if h.current > h.max {
            h.current = h.max;
        }
    }
}
// consumes DamageEvent -> mutates health, DeathEvent emitted, if needed
pub fn apply_damage_events(
    mut reader: MessageReader<DamageEvent>,
    mut writer_death: MessageWriter<DeathEvent>,
    mut q: Query<&mut Health>,
) {
    for ev in reader.read() {
        if let Ok(mut h) = q.get_mut(ev.target) {
            if h.i_frames > 0.0 || h.is_dead() {
                continue;
            }
            h.current = (h.current - ev.amount.max(0.0)).max(0.0);
            if h.is_dead() {
                writer_death.write(DeathEvent { entity: ev.target });
            }
        }
    }
}

// consumes HealEvent -> increases Health (no heal beyond max)
pub fn apply_heal_events(mut reader: MessageReader<HealEvent>, mut q: Query<&mut Health>) {
    for ev in reader.read() {
        if let Ok(mut h) = q.get_mut(ev.target) {
            if !h.is_dead() {
                h.current = (h.current + ev.amount.max(0.0)).min(h.max); // not to go over the max
            }
        }
    }
}

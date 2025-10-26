use crate::{
    events::*,
    prelude::{CircleCollider, Health, Lifetime, Velocity},
    systems::*,
};
use bevy::prelude::*;

// system sets for explicit ordering
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CoreSet {
    PrePhysics, // pure input/ ai run before movement
    Simulation, // kinematics & health pipeline
    Post,       // cleanup/desapwn
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Velocity>()
            .register_type::<CircleCollider>()
            .register_type::<Lifetime>()
            .add_message::<DamageEvent>()
            .add_message::<HealEvent>()
            .add_message::<DeathEvent>()
            // system sets for organization
            .configure_sets(
                Update,
                (
                    CoreSet::PrePhysics,
                    CoreSet::Simulation.after(CoreSet::PrePhysics),
                    CoreSet::Post.after(CoreSet::Simulation),
                ),
            )
            // movement & kinematics
            .add_systems(Update, apply_velocity.in_set(CoreSet::Simulation))
            // health pipeline
            .add_systems(
                Update,
                (tick_health, apply_damage_events, apply_heal_events).in_set(CoreSet::Simulation),
            )
            // lifetime & death cleanup
            .add_systems(
                Update,
                (tick_lifetimes, despawn_on_death).in_set(CoreSet::Post),
            );
    }
}

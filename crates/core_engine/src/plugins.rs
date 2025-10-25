use crate::{events::*, systems::*};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageEvent>();
        // .add_systems(Update, ());
    }
}

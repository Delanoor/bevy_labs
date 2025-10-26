use crate::components::Velocity;
use bevy::prelude::*;

/// Applies Velocity to Transform each frame with optional drag.

pub fn apply_velocity(mut q: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut tf, mut vel) in q.iter_mut() {
        tf.translation.x += vel.lin_vel.x * dt;
        tf.translation.y += vel.lin_vel.y * dt;

        if vel.drag > 0.0 {
            // exponential decay: v *= (1 - drag)^dt

            let factor = (1.0 - vel.drag).powf(dt.max(0.0)).clamp(0.0, 1.0);
            vel.lin_vel *= factor;

            if vel.lin_vel.length_squared() < 1e-6 {
                vel.lin_vel = Vec2::ZERO;
            }
        }
    }
}

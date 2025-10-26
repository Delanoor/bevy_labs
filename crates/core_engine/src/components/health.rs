use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,

    // post-hit invulnerability window (seconds remainin)
    pub i_frames: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            i_frames: 0.0,
        }
    }

    pub fn with_invuln(max: f32, i_frames: f32) -> Self {
        Self {
            current: max,
            max,
            i_frames,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
    pub fn ratio(&self) -> f32 {
        if self.max > 0.0 {
            (self.current / self.max).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn set_max(&mut self, new_max: f32, keep_ratio: bool) {
        let r = self.ratio();
        self.max = new_max.max(1.0); // ensure it's at least 1.0
        self.current = if keep_ratio {
            self.max * r
        } else {
            self.current.min(self.max)
        }
    }
}

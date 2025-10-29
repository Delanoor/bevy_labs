use std::time::Duration;

use bevy::{
    color::palettes::tailwind::{BLUE_300, RED_300},
    prelude::*,
};
use core_engine::prelude::{CircleCollider, CorePlugin, Health, Lifetime, Velocity};

const HALF_W: f32 = 480.0;
const HALF_H: f32 = 270.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .insert_resource(Bounds {
            half_w: HALF_W,
            half_h: HALF_H,
        })
        .insert_resource(SpawnTuning::default())
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(
            Update,
            (player_input, clamp_player, spawn_hazards, end_if_collision),
        )
        .run();
}

#[derive(Resource)]
struct Bounds {
    half_w: f32,
    half_h: f32,
}

#[derive(Resource)]
struct SpawnTuning {
    timer: Timer,
    fall_speed: f32,
}

impl Default for SpawnTuning {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.8, TimerMode::Repeating),
            fall_speed: -260.0,
        }
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Hazard;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(24.0)),
            color: Color::Srgba(RED_300),

            ..default()
        },
        Health::new(1.0),
        Velocity::with_drag(Vec2::ZERO, 0.8),
        CircleCollider::new(12.0),
        Transform::from_xyz(0.0, -HALF_H, 0.0),
        Player,
    ));
}

fn player_input(kb: Res<ButtonInput<KeyCode>>, mut q: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut v) = q.single_mut() {
        let mut dir = Vec2::ZERO;

        if kb.pressed(KeyCode::KeyA) || kb.pressed(KeyCode::ArrowLeft) {
            dir.x -= 1.0;
        }

        if kb.pressed(KeyCode::KeyD) || kb.pressed(KeyCode::ArrowRight) {
            dir.x += 1.0;
        }

        let speed = 300.0;

        // v.lin_vel = if dir == Vec2::ZERO {
        //     Vec2::ZERO
        // } else {
        //     dir.normalize() * speed
        // };

        // only apply when theres a directional change
        if dir != Vec2::ZERO {
            v.lin_vel = dir.normalize() * speed
        };
    }
}
fn clamp_player(
    bounds: Res<Bounds>,
    mut q: Query<(&mut Transform, &mut Velocity, &CircleCollider), With<Player>>,
) {
    for (mut t, mut v, c) in q.iter_mut() {
        let r = c.radius;

        let (min_x, max_x) = (-bounds.half_w + r, bounds.half_w - r);

        // if the player touches
        if t.translation.x < min_x {
            t.translation.x = min_x;
            v.lin_vel.x *= -1.0;
        } else if t.translation.x > max_x {
            t.translation.x = max_x;
            v.lin_vel.x *= -1.0;
        }
    }
}
fn spawn_hazards(
    mut commands: Commands,
    time: Res<Time>,
    bounds: Res<Bounds>,
    mut tune: ResMut<SpawnTuning>,
) {
    // speeding up spawn over time, min 0.25s
    tune.timer.tick(time.delta());
    if tune.timer.just_finished() {
        use rand::Rng;
        let mut rng = rand::rng();
        let x = rng.random_range(-bounds.half_w + 16.0..bounds.half_w - 16.0);

        commands.spawn((
            Hazard,
            Velocity::new(Vec2::new(0.0, tune.fall_speed)),
            CircleCollider::new(12.0),
            Lifetime::seconds(6.0),
            Transform::from_xyz(x, bounds.half_h + 20.0, 0.0),
            Sprite {
                custom_size: Some(Vec2::splat(20.0)),
                color: Color::Srgba(BLUE_300),

                ..default()
            },
        ));

        let s = tune.timer.duration().as_secs_f32();
        let new_s = (s * 0.97).max(0.25);
        tune.timer.set_duration(Duration::from_secs_f32(new_s));

        // increase fall speed
        tune.fall_speed -= 5.0;
    }
}

fn end_if_collision() {}

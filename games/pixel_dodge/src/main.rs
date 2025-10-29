use bevy::{color::palettes::tailwind::RED_300, prelude::*};
use core_engine::prelude::{CircleCollider, CorePlugin, Health, Velocity};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .insert_resource(Bounds {
            half_w: 480.0,
            half_h: 270.0,
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
        Velocity::default(),
        CircleCollider::new(12.0),
        Player,
    ));
}

fn player_input(kb: Res<ButtonInput<KeyCode>>, mut q: Query<&mut Velocity, With<Player>>) {}
fn clamp_player() {}
fn spawn_hazards() {}

fn end_if_collision() {}

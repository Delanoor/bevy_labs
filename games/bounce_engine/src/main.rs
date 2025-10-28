use bevy::{color::palettes::tailwind::CYAN_400, prelude::*};
use core_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .insert_resource(Gravity(Vec2::new(0.0, -900.0)))
        .insert_resource(Bounds {
            half_w: 480.0,
            half_h: 270.0,
        })
        .add_systems(Startup, (setup_camera, spawn_balls))
        .add_systems(Update, (apply_gravity, bounce_off_walls).chain())
        .run();
}

#[derive(Resource)]
struct Gravity(Vec2);

#[derive(Resource)]
struct Bounds {
    half_w: f32,
    half_h: f32,
}

#[derive(Component)]
struct Ball;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_balls(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(12., 12.)),
            color: Color::Srgba(CYAN_400),
            ..default()
        },
        Ball,
    ));
}

fn apply_gravity() {
    info!("Gravity");
}

fn bounce_off_walls() {
    info!("Bounce");
}

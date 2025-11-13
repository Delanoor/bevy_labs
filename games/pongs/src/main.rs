use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2); // to represent logical postion

#[derive(Component)] // marker component
#[require(Position)]
struct Ball;

fn spawn_ball(mut commands: Commands) {
    commands.spawn((Position(Vec2::ZERO), Ball));
}

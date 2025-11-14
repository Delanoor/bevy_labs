use bevy::{prelude::*, sprite_render::Material2d};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_ball))
        .add_systems(
            Update,
            (move_ball.before(project_positions), project_positions),
        )
        .run();
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2); // to represent logical postion

#[derive(Component)] // marker component
#[require(Position)]
struct Ball;

const BALL_SIZE: f32 = 30.0;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Assets::add will load these into memory and return a Handle (like an Id)

    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);
    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut tf, position) in &mut positionables {
        tf.translation = position.0.extend(0.);
    }
}

fn move_ball(mut position: Single<&mut Position, With<Ball>>) {
    // if let Ok(mut position) = ball.single_mut() {
    //     position.0.x += 1.0
    // }
    position.0.x += 1.0;
}

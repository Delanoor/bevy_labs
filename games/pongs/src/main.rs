use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_ball, spawn_paddles))
        .add_systems(
            FixedUpdate,
            (
                move_ball.before(project_positions),
                project_positions,
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}

#[derive(Component)]
struct Collider(Rectangle);

const BALL_SPEED: f32 = 2.;
#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2); // to represent logical postion

#[derive(Component)] // marker component
#[require(Position, Velocity = Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)), Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)))]
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

fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();

    position.0 += velocity.0 * BALL_SPEED;
}

// ====================== Paddle ======================

const PADDLE_SHAPE: Rectangle = Rectangle::new(20., 70.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);
const PADDLE_AI_COLOR: Color = Color::srgb(0., 0., 1.);

#[derive(Component)]
#[require(Position, Collider = Collider(PADDLE_SHAPE))]
struct Paddle;

#[derive(Component)]
struct Player;

// other paddle
#[derive(Component)]
struct Ai;

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);
    let material_ai = materials.add(PADDLE_AI_COLOR);

    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.0;
    let player_position = Vec2::new(half_window_size.x - padding, 0.);

    // ===================== player =====================
    commands.spawn((
        Player,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material),
        Position(player_position),
    ));

    // ===================== AI =====================
    let ai_position = Vec2::new(-half_window_size.x + padding, 0.);
    commands.spawn((
        Ai,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_ai),
        Position(ai_position),
    ));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

fn handle_collisions(
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider) in &other_things {
        if let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) {
            match collision {
                Collision::Left => {
                    ball_velocity.0.x *= -1.;
                }
                Collision::Right => {
                    ball_velocity.0.x *= -1.;
                }
                Collision::Top => {
                    ball_velocity.0.y *= -1.;
                }
                Collision::Bottom => {
                    ball_velocity.0.y *= -1.;
                }
            }
        }
    }
}

use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score { player: 0, ai: 0 })
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_ball,
                spawn_paddles,
                spawn_gutters,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                project_positions,
                move_ball.before(project_positions),
                handle_collisions.after(move_ball),
                move_paddles.before(project_positions),
                handle_player_input.before(move_paddles),
                constrain_paddle_position.after(move_paddles),
                detect_goal.after(move_ball),
                update_scoreboard,
                move_ai,
            ),
        )
        .add_observer(reset_ball)
        .add_observer(update_score)
        .run();
}

#[derive(Component, Default)]
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
        Velocity(Vec2::ZERO),
    ));

    // ===================== AI =====================
    let ai_position = Vec2::new(-half_window_size.x + padding, 0.);
    commands.spawn((
        Ai,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material_ai),
        Position(ai_position),
        Velocity(Vec2::ZERO),
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

// Swept collision detection: checks if a ray intersects an AABB
// Returns (collision_time, collision_normal) if collision occurs within [0, 1]
fn sweep_ray_vs_aabb(ray_origin: Vec2, ray_direction: Vec2, aabb: Aabb2d) -> Option<(f32, Vec2)> {
    if ray_direction.length_squared() < 0.0001 {
        return None; // Not moving
    }

    let aabb_min = aabb.min;
    let aabb_max = aabb.max;

    // Calculate intersection times for each axis
    let inv_dir = Vec2::new(1.0 / ray_direction.x, 1.0 / ray_direction.y);

    let t1 = (aabb_min.x - ray_origin.x) * inv_dir.x;
    let t2 = (aabb_max.x - ray_origin.x) * inv_dir.x;
    let t3 = (aabb_min.y - ray_origin.y) * inv_dir.y;
    let t4 = (aabb_max.y - ray_origin.y) * inv_dir.y;

    let tmin_x = t1.min(t2);
    let tmax_x = t1.max(t2);
    let tmin_y = t3.min(t4);
    let tmax_y = t3.max(t4);

    // Find the entry and exit times
    let t_enter = tmin_x.max(tmin_y);
    let t_exit = tmax_x.min(tmax_y);

    // Check if there's a valid intersection within [0, 1]
    if t_enter > t_exit || t_exit < 0.0 || t_enter > 1.0 {
        return None;
    }

    let collision_time = t_enter.max(0.0);

    // Determine collision normal based on which face we hit
    let normal = if tmin_x > tmin_y {
        if t1 > t2 {
            Vec2::new(1.0, 0.0) // Hit left face
        } else {
            Vec2::new(-1.0, 0.0) // Hit right face
        }
    } else {
        if t3 > t4 {
            Vec2::new(0.0, 1.0) // Hit bottom face
        } else {
            Vec2::new(0.0, -1.0) // Hit top face
        }
    };

    Some((collision_time, normal))
}

fn handle_collisions(
    ball: Single<(&mut Velocity, &mut Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>,
) {
    let (mut ball_velocity, mut ball_position, ball_collider) = ball.into_inner();

    let old_pos = ball_position.0;
    let movement = ball_velocity.0 * BALL_SPEED;
    let ball_radius = ball_collider.half_size().x;

    let mut closest_collision: Option<(f32, Vec2)> = None;

    for (other_position, other_collider) in &other_things {
        // Expand the collider by the ball's radius (Minkowski sum)
        let expanded_bounds = Aabb2d::new(
            other_position.0,
            other_collider.half_size() + Vec2::splat(ball_radius),
        );

        // Check if the ball's path intersects this expanded collider
        if let Some((time, normal)) = sweep_ray_vs_aabb(old_pos, movement, expanded_bounds) {
            // Keep track of the earliest collision
            if closest_collision.is_none() || time < closest_collision.unwrap().0 {
                closest_collision = Some((time, normal));
            }
        }
    }

    // Handle the closest collision
    if let Some((collision_time, normal)) = closest_collision {
        // Move ball to collision point (slightly before to prevent overlap)
        ball_position.0 = old_pos + movement * collision_time * 0.999;

        // Reflect velocity based on the collision normal
        let dot = ball_velocity.0.dot(normal);
        ball_velocity.0 = ball_velocity.0 - 2.0 * dot * normal;

        // Apply speed increase
        ball_velocity.0 *= 1.1;
    }
}

// ===================== Gutter =====================

#[derive(Component)]
#[require(Position, Collider)]
struct Gutter;

const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
const GUTTER_HEIGHT: f32 = 20.;

fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let material = materials.add(GUTTER_COLOR);
    let padding = 20.;

    let gutter_shape = Rectangle::new(window.resolution.width(), GUTTER_HEIGHT);
    let mesh = meshes.add(gutter_shape);

    let top_gutter_position = Vec2::new(0., window.resolution.height() / 2. - padding);

    commands.spawn((
        Gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(top_gutter_position),
        Collider(gutter_shape),
    ));

    let bottom_gutter_position = Vec2::new(0., -window.resolution.height() / 2. + padding);
    commands.spawn((
        Gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position(bottom_gutter_position),
        Collider(gutter_shape),
    ));
}

// ===================== Movement =====================

const PADDLE_SPEED: f32 = 9.;

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        paddle_velocity.0.y = PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        paddle_velocity.0.y = -PADDLE_SPEED;
    } else {
        paddle_velocity.0.y = 0.;
    }
}

fn move_paddles(mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>) {
    for (mut pos, vel) in &mut paddles {
        pos.0 += vel.0;
    }
}

fn constrain_paddle_position(
    mut paddles: Query<(&mut Position, &Collider), (With<Paddle>, Without<Gutter>)>,
    gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>)>,
) {
    for (mut paddle_position, paddle_collider) in &mut paddles {
        for (gutter_position, gutter_collider) in &gutters {
            let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
            let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

            if let Some(collision) = collide_with_side(paddle_aabb, gutter_aabb) {
                match collision {
                    Collision::Top => {
                        paddle_position.0.y = gutter_position.0.y
                            + gutter_collider.half_size().y
                            + paddle_collider.half_size().y;
                    }
                    Collision::Bottom => {
                        paddle_position.0.y = gutter_position.0.y
                            - gutter_collider.half_size().y
                            - paddle_collider.half_size().y;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[derive(Resource)]
struct Score {
    player: u32,
    ai: u32,
}

// there are two types of Events
// Event - global
// EntityEvent events related to a specific entity
#[derive(EntityEvent)]
struct Scored {
    #[event_target]
    scorer: Entity,
}

fn detect_goal(
    ball: Single<(&Position, &Collider), With<Ball>>,
    player: Single<Entity, (With<Player>, Without<Ai>)>,
    ai: Single<Entity, (With<Ai>, Without<Player>)>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    let (ball_position, ball_collider) = ball.into_inner();
    let half_window_size = window.resolution.size() / 2.;

    if ball_position.0.x - ball_collider.half_size().x > half_window_size.x {
        commands.trigger(Scored { scorer: *player })
    }

    if ball_position.0.x + ball_collider.half_size().x < -half_window_size.x {
        commands.trigger(Scored { scorer: *ai })
    }
}

fn reset_ball(_event: On<Scored>, ball: Single<(&mut Position, &mut Velocity), With<Ball>>) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    ball_position.0 = Vec2::ZERO;
    ball_velocity.0 = Vec2::new(BALL_SPEED, 0.);
}

fn update_score(
    event: On<Scored>,
    mut score: ResMut<Score>,
    is_ai: Query<&Ai>,
    is_player: Query<&Player>,
) {
    if is_ai.get(event.scorer).is_ok() {
        score.ai += 1;
        info!("AI scored: {} - {}", score.player, score.ai);
    }

    if is_player.get(event.scorer).is_ok() {
        score.player += 1;
        info!("Player scored: {} - {}", score.player, score.ai);
    }
}

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

fn spawn_scoreboard(mut commands: Commands) {
    let container = Node {
        width: percent(100.0),
        height: percent(100.0),
        justify_content: JustifyContent::Center,
        ..default()
    };

    // Then add a container for the text
    let header = Node {
        width: px(200.),
        height: px(100.),
        ..default()
    };

    // The players score on the left hand side
    let player_score = (
        PlayerScore,
        Text::new("0"),
        TextFont::from_font_size(72.0),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.0),
            left: px(25.0),
            ..default()
        },
    );

    // The AI score on the right hand side
    let ai_score = (
        AiScore,
        Text::new("0"),
        TextFont::from_font_size(72.0),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            top: px(5.0),
            right: px(25.0),
            ..default()
        },
    );

    commands.spawn((
        container,
        children![(header, children![player_score, ai_score])],
    ));
}

fn update_scoreboard(
    mut player_score: Single<&mut Text, (With<PlayerScore>, Without<AiScore>)>,
    mut ai_score: Single<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        player_score.0 = score.player.to_string();
        ai_score.0 = score.ai.to_string();
    }
}

fn move_ai(ai: Single<(&mut Velocity, &Position), With<Ai>>, ball: Single<&Position, With<Ball>>) {
    let (mut velocity, position) = ai.into_inner();
    let a_to_b = ball.0 - position.0;

    velocity.0.y = a_to_b.y.signum() * PADDLE_SPEED;
}

// ============================================================================
// RAPIER ALTERNATIVE IMPLEMENTATION (for reference/comparison)
// ============================================================================
// To use this approach, add to Cargo.toml:
// [dependencies]
// bevy_rapier2d = "0.27"  # Check latest version for Bevy 0.17
//
// Then replace the imports and main function setup below:

/*
use bevy_rapier2d::prelude::*;

fn main_with_rapier() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())  // Optional: visual debugging
        .insert_resource(Score { player: 0, ai: 0 })
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_ball_rapier,
                spawn_paddles_rapier,
                spawn_gutters_rapier,
                spawn_scoreboard,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                // No need for: project_positions, move_ball, handle_collisions
                // Rapier does this automatically!
                handle_player_input_rapier,
                detect_goal,  // Keep our custom goal detection
                update_scoreboard,
                move_ai_rapier,
            ),
        )
        .add_observer(reset_ball_rapier)
        .add_observer(update_score)
        .run();
}

// Spawn ball with Rapier components
fn spawn_ball_rapier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    commands.spawn((
        Ball,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        // Rapier components replace Position + Velocity + Collider
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE / 2.0),
        Velocity::linear(Vec2::new(BALL_SPEED * 60.0, BALL_SPEED * 60.0)),  // Rapier uses pixels/sec
        Restitution::coefficient(1.0),  // Perfectly elastic collisions
        Friction::coefficient(0.0),     // No friction
        GravityScale(0.0),              // No gravity in pong
        Ccd::enabled(),                 // ← THIS IS THE KEY: Continuous Collision Detection!
    ));
}

// Spawn paddles with Rapier components
fn spawn_paddles_rapier(
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

    // Player paddle
    commands.spawn((
        Player,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material),
        Transform::from_xyz(half_window_size.x - padding, 0., 0.),
        RigidBody::KinematicVelocityBased,  // Controlled by code, not physics
        Collider::cuboid(PADDLE_SHAPE.half_size.x, PADDLE_SHAPE.half_size.y),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // AI paddle
    commands.spawn((
        Ai,
        Paddle,
        Mesh2d(mesh),
        MeshMaterial2d(material_ai),
        Transform::from_xyz(-half_window_size.x + padding, 0., 0.),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(PADDLE_SHAPE.half_size.x, PADDLE_SHAPE.half_size.y),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));
}

// Spawn gutters with Rapier components
fn spawn_gutters_rapier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let material = materials.add(GUTTER_COLOR);
    let padding = 20.;
    let gutter_shape = Rectangle::new(window.resolution.width(), GUTTER_HEIGHT);
    let mesh = meshes.add(gutter_shape);

    // Top gutter
    commands.spawn((
        Gutter,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0., window.resolution.height() / 2. - padding, 0.),
        RigidBody::Fixed,  // Immovable
        Collider::cuboid(gutter_shape.half_size.x, gutter_shape.half_size.y),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));

    // Bottom gutter
    commands.spawn((
        Gutter,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0., -window.resolution.height() / 2. + padding, 0.),
        RigidBody::Fixed,
        Collider::cuboid(gutter_shape.half_size.x, gutter_shape.half_size.y),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));
}

// Player input with Rapier (writes to Rapier's Velocity component)
fn handle_player_input_rapier(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Single<&mut Velocity, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        paddle.linvel.y = PADDLE_SPEED * 60.0;  // Convert to pixels/sec
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        paddle.linvel.y = -PADDLE_SPEED * 60.0;
    } else {
        paddle.linvel.y = 0.;
    }
}

// AI movement with Rapier
fn move_ai_rapier(
    mut ai: Single<(&mut Velocity, &Transform), With<Ai>>,
    ball: Single<&Transform, With<Ball>>,
) {
    let (mut velocity, transform) = ai.into_inner();
    let a_to_b = ball.translation.truncate() - transform.translation.truncate();
    velocity.linvel.y = a_to_b.y.signum() * PADDLE_SPEED * 60.0;
}

// Reset ball with Rapier
fn reset_ball_rapier(
    _event: On<Scored>,
    mut ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let (mut transform, mut velocity) = ball.into_inner();
    transform.translation = Vec3::ZERO;
    velocity.linvel = Vec2::new(BALL_SPEED * 60.0, 0.);
}

// For ball acceleration on collision with Rapier:
fn accelerate_ball_on_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut ball_velocity: Query<&mut Velocity, With<Ball>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            // Check if one of the entities is the ball
            if let Ok(mut vel) = ball_velocity.get_mut(*entity1) {
                vel.linvel *= 1.1;
            } else if let Ok(mut vel) = ball_velocity.get_mut(*entity2) {
                vel.linvel *= 1.1;
            }
        }
    }
}
*/

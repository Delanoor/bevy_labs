use core_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin) // movement, lifetime, damage,
        .insert_resource(Score::default())
        .insert_resource(RoundTimer { time_left: 60.0 })
        .add_systems(Startup, (setup_camera, spawn_player, spawn_ui))
        .add_systems(
            Update,
            (
                player_input,
                clamp_bounds,
                spawn_target_periodically,
                collect_targets,
                update_hud,
                tick_round,
            ),
        )
        .run();
}

const ARENA_HALF_W: f32 = 480.0;
const ARENA_HALF_H: f32 = 270.0;

#[derive(Default, Resource)]
struct Score(u32);

#[derive(Resource)]
struct RoundTimer {
    time_left: f32,
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Target;
#[derive(Component)]
struct HudText;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("cat.png");
    commands.spawn((
        Player,
        Health::new(1.0),
        Velocity::default(),
        CircleCollider::new(70.0),
        Sprite {
            image,
            custom_size: Some(Vec2::new(120., 70.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            ..default()
        },
    ));
}

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Block,
            position_type: PositionType::Absolute,
            top: Val::Px(40.),
            right: Val::Px(100.),
            ..default()
        },
        Text::new("0"),
        TextFont {
            font_size: 60.0,
            ..Default::default()
        },
        HudText,
    ));
}

fn player_input(kb: Res<ButtonInput<KeyCode>>, mut q: Query<&mut Velocity, With<Player>>) {
    let mut dir = Vec2::ZERO;

    // Up movement
    if kb.pressed(KeyCode::KeyW) || kb.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    // Down movement
    if kb.pressed(KeyCode::KeyS) || kb.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    // Right movement
    if kb.pressed(KeyCode::KeyD) || kb.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    // Left movement
    if kb.pressed(KeyCode::KeyA) || kb.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }

    // Apply movement to player
    if let Ok(mut velocity) = q.single_mut() {
        let speed = 260.0;
        velocity.lin_vel = dir.normalize_or_zero() * speed
    }
}
fn clamp_bounds(mut q: Query<&mut Transform, With<Player>>) {
    if let Ok(mut t) = q.single_mut() {
        t.translation.x = t.translation.x.clamp(-ARENA_HALF_W, ARENA_HALF_W);
        t.translation.y = t.translation.y.clamp(-ARENA_HALF_H, ARENA_HALF_H);
    }
}

#[derive(Resource, Default)]
struct SpawnClock(Timer);
fn spawn_target_periodically(
    mut commands: Commands,
    time: Res<Time>,
    mut clock: Local<SpawnClock>,
    existing: Query<Entity, With<Target>>,
) {
    if clock.0.duration().is_zero() {
        clock.0 = Timer::from_seconds(2.0, TimerMode::Repeating);
    }

    clock.0.tick(time.delta());
    if clock.0.just_finished() && existing.is_empty() {
        use rand::Rng;
        let mut rng = rand::rng();
        let x = rng.random_range(-ARENA_HALF_W + 20.0..ARENA_HALF_W - 20.0);
        let y = rng.random_range(-ARENA_HALF_H + 20.0..ARENA_HALF_H - 20.0);

        commands.spawn((
            Target,
            CircleCollider::new(10.0),
            Lifetime::seconds(5.0), // CorePlugin ticks and despawns
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),        // Red color
                custom_size: Some(Vec2::new(20.0, 20.0)), // 20x20 square
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}
fn collect_targets(
    mut commands: Commands,
    mut score: ResMut<Score>,
    p_q: Query<(&Transform, &CircleCollider), With<Player>>,
    t_q: Query<(Entity, &Transform, &CircleCollider), With<Target>>,
) {
    if let Ok((pt, pc)) = p_q.single() {
        let p = pt.translation.truncate(); // discard z 
        for (e, tt, tc) in t_q.iter() {
            let d2 = p.distance_squared(tt.translation.truncate());
            let r = pc.radius + tc.radius;
            if d2 <= r * r {
                score.0 += 1;
                commands.entity(e).despawn();
                break;
            }
        }
    }
}
fn update_hud(score: Res<Score>, round: Res<RoundTimer>, mut q: Query<&mut Text, With<HudText>>) {
    if !score.is_changed() && !round.is_changed() {
        return;
    }

    if let Ok(mut text) = q.single_mut() {
        let secs = round.time_left.max(0.0).floor() as i32;
        text.0 = format!("Score: {} \n Time: {}", score.0, secs);
    }
}
fn tick_round(time: Res<Time>, mut round: ResMut<RoundTimer>) {
    round.time_left -= time.delta_secs();
    if round.time_left <= 0.0 {
        round.time_left = 0.0;
        info!("Round over.") // TODO: state change
    }
}

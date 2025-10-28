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
    use rand::Rng;

    let mut rng = rand::rng();

    for _ in 0..12 {
        let x = rng.random_range(-420.0..420.0);
        let y = rng.random_range(50.0..220.0);
        let vx = rng.random_range(-120.0..120.0);
        let vy = rng.random_range(20.0..180.0);

        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::splat(24.0)),
                color: Color::Srgba(CYAN_400),
                ..default()
            },
            Ball,
            CircleCollider::new(12.0),
            Transform::from_xyz(x, y, 0.0),
            Velocity::with_drag(Vec2::new(vx, vy), 0.05),
        ));
    }
}

fn apply_gravity(mut q: Query<&mut Velocity, With<Ball>>, g: Res<Gravity>, time: Res<Time>) {
    let dt = time.delta_secs();

    for mut v in &mut q {
        v.lin_vel += g.0 * dt;
    }
}

fn bounce_off_walls(
    bounds: Res<Bounds>,
    mut q: Query<(&mut Transform, &mut Velocity, &CircleCollider), With<Ball>>,
) {
    for (mut t, mut v, c) in &mut q {
        let r = c.radius;
        let (min_x, max_x) = (-bounds.half_w + r, bounds.half_w - r);
        let (min_y, max_y) = (-bounds.half_h + r, bounds.half_h - r);

        // FOR X
        if t.translation.x < min_x {
            t.translation.x = min_x; // clamp to x-boundary
            v.lin_vel.x = -v.lin_vel.x * 0.9; // bounce back
        } else if t.translation.x > max_x {
            t.translation.x = max_x;
            v.lin_vel.x = -v.lin_vel.x * 0.9;
        }

        // FOR Y
        if t.translation.y < min_y {
            t.translation.y = min_y;
            v.lin_vel.y = -v.lin_vel.y * 0.8; // a bit softer  -> gradually decreases the bounce
        } else if t.translation.y > max_y {
            t.translation.y = max_y;
            v.lin_vel.y = -v.lin_vel.y * 0.9;
        }
    }
}

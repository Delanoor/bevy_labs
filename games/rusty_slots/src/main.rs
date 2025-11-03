use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Credits(100))
        .insert_resource(Bet(5))
        .insert_resource(SpinLock(false))
        .add_systems(Startup, (setup_camera, spawn_ui, spawn_reels))
        .add_systems(
            Update,
            (
                spin_button,
                drive_spinning,
                ease_to_stop,
                settle_and_payout,
                update_ui,
            ),
        )
        .run();
}

// ======================== Resource =========================

#[derive(Resource)]
struct Credits(pub i32);
#[derive(Resource)]
struct Bet(pub i32);
#[derive(Resource)]
struct SpinLock(pub bool); // to disable button when spinning

// ======================== Components =========================
#[derive(Component)]
struct Reel {
    idx: usize,
    rows: usize,
}
#[derive(Component)]
struct SpinTimer(Option<Timer>);
#[derive(Component)]
struct ReelSpeed(pub f32);
#[derive(Component)]
struct ReelOffset(pub f32);
#[derive(Component)]
struct Stopping {
    target: i32,
    ease: Timer,
}

#[derive(Component)]
struct BtnSpin;
#[derive(Component)]
struct UiCredits;
#[derive(Component)]
struct UiBet;
#[derive(Component)]
struct UiResult;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn spawn_ui(mut commands: Commands) {
    // spin button
    commands.spawn((Button, BtnSpin));
}

fn spawn_reels(mut commands: Commands) {
    // 3reels each with 6 symbols
    let rows = 6;
    for i in 0..3 {
        commands.spawn((
            Reel { idx: i, rows },
            SpinTimer(None),
            ReelSpeed(8.0 + i as f32 * 1.5), // all different speeds, slightly
            ReelOffset(0.0),
            Sprite {
                custom_size: Some(Vec2::new(80.0, 200.0)),
                ..default()
            },
        ));
    }
}

fn spin_button(
    mut q_btn: Query<&Interaction, (Changed<Interaction>, With<BtnSpin>)>,
    mut reels: Query<(&mut SpinTimer, &mut ReelOffset, &mut ReelSpeed, &Reel)>,
    mut lock: ResMut<SpinLock>,
    mut credits: ResMut<Credits>,
    bet: Res<Bet>,
) {
    for interaction in &mut q_btn {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if lock.0 || credits.0 < bet.0 {
            return;
        }

        // lock and pay bet
        lock.0 = true;
        credits.0 -= bet.0;

        for (mut t, mut off, mut spd, _r) in &mut reels {
            *off = ReelOffset(0.0);
            t.0 = Some(Timer::from_seconds(3.0, TimerMode::Repeating)); // running 
            // randomize speed later
            spd.0 *= 1.0;
        }
    }
}
fn drive_spinning(
    time: Res<Time>,
    mut q: Query<(&mut SpinTimer, &mut ReelOffset, &Reel, Option<&Stopping>)>,
) {
    for (mut timer, mut off, reel, stopping) in &mut q {
        // if easing to a stop, skip raw spin here
        if stopping.is_some() {
            continue;
        }

        if let Some(t) = timer.0.as_mut() {
            t.tick(time.delta());

            let rows_per_sec = 12.0;
            off.0 = (off.0 + rows_per_sec * time.delta_secs()) % reel.rows as f32;
        }
    }
}
fn ease_to_stop() {}
fn settle_and_payout() {}
fn update_ui(
    credits: Res<Credits>,
    bet: Res<Bet>,
    mut queries: ParamSet<(
        Query<&mut Text, With<UiCredits>>,
        Query<&mut Text, With<UiBet>>,
    )>,
) {
    if let Ok(mut t) = queries.p0().single_mut() {
        t.0 = format!("Credits: {}", credits.0);
    }
    if let Ok(mut t) = queries.p1().single_mut() {
        t.0 = format!("Bet: {}", bet.0);
    }
}

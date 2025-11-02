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

fn spin_button() {}
fn drive_spinning() {}
fn ease_to_stop() {}
fn settle_and_payout() {}
fn update_ui() {}

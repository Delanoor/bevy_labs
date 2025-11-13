use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .init_resource::<InputFocus>()
        .insert_resource(Credits(100))
        .insert_resource(Bet(5))
        .insert_resource(SpinLock(false))
        .add_systems(Startup, (setup_camera, spawn_ui, spawn_reels))
        .add_systems(
            Update,
            (spin_button, drive_spinning, ease_to_stop, settle_and_payout),
        )
        .add_systems(
            Update,
            update_ui.after(spin_button).after(settle_and_payout),
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
fn spawn_ui(mut commands: Commands, credits: Res<Credits>, bet: Res<Bet>) {
    // spin button
    commands.spawn(button(credits, bet));
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
                color: Color::Hsla(Hsla {
                    hue: (i as f32 + 30.0),
                    saturation: (1.0),
                    lightness: (i as f32 * 0.3),
                    alpha: (1.0),
                }),
                custom_size: Some(Vec2::new(80.0, 200.0)),
                ..default()
            },
            Transform::from_xyz(-80.0 + i as f32 * 80.0, 0.0, 0.0),
        ));
    }
}

fn spin_button(
    q_btn: Query<(&Interaction, &Children), (Changed<Interaction>, With<BtnSpin>)>,
    mut q_text: Query<&mut Text>,
    mut reels: Query<(&mut SpinTimer, &mut ReelOffset, &mut ReelSpeed, &Reel)>,
    mut lock: ResMut<SpinLock>,
    mut credits: ResMut<Credits>,
    bet: Res<Bet>,
) {
    for (interaction, children) in &q_btn {
        // Update button text based on interaction state
        for child in children.iter() {
            if let Ok(mut text) = q_text.get_mut(child) {
                match *interaction {
                    Interaction::Hovered => continue,
                    Interaction::Pressed => text.0 = "GOOD LUCK!".to_string(),
                    Interaction::None => text.0 = "SPIN".to_string(),
                }
            }
        }

        // Handle spin logic only on press
        if *interaction != Interaction::Pressed {
            continue;
        }

        println!(
            "Button pressed! Lock: {}, Credits: {}, Bet: {}",
            lock.0, credits.0, bet.0
        );

        if lock.0 || credits.0 < bet.0 {
            println!("Blocked: lock={} or insufficient credits", lock.0);
            return;
        }

        // lock and pay bet
        lock.0 = true;
        credits.0 -= bet.0;

        for (mut t, mut off, mut spd, _r) in &mut reels {
            *off = ReelOffset(0.0);
            t.0 = Some(Timer::from_seconds(3.0, TimerMode::Once)); // spin for 3 seconds
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

            // Clear timer when finished
            if t.is_finished() {
                timer.0 = None;
                println!("Reel finished spinning");
            }
        }
    }
}
fn ease_to_stop() {}
fn settle_and_payout(mut lock: ResMut<SpinLock>, q_timers: Query<&SpinTimer>) {
    // Check if any reels are still spinning
    let any_spinning = q_timers.iter().any(|timer| timer.0.is_some());

    // If no reels are spinning and we're locked, unlock
    if !any_spinning && lock.0 {
        lock.0 = false;
        println!("Spin complete - unlocked!");
    }
}
fn update_ui(
    credits: Res<Credits>,
    bet: Res<Bet>,
    mut queries: ParamSet<(
        Query<&mut Text, With<UiCredits>>,
        Query<&mut Text, With<UiBet>>,
    )>,
) {
    // Only update if credits changed
    if credits.is_changed() {
        if let Ok(mut t) = queries.p0().single_mut() {
            println!("Updating UI text to: Credits: {}", credits.0);
            t.0 = format!("Credits: {}", credits.0);
        } else {
            println!("Failed to get credits text query!");
        }
    }
    // Only update if bet changed
    if bet.is_changed() {
        if let Ok(mut t) = queries.p1().single_mut() {
            t.0 = format!("Bet: {}", bet.0);
        }
    }
}

fn button(credits: Res<Credits>, bet: Res<Bet>) -> impl Bundle {
    let init_credits = format!("Credits: {}", credits.0);
    let init_bet = format!("Bet: {}", bet.0);
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Button,
                BtnSpin,
                Node {
                    width: px(150),
                    height: px(65),
                    border: UiRect::all(px(5)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BorderRadius::MAX,
                BackgroundColor(Color::BLACK),
                children![(
                    Text::new("SPIN"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ),
            (
                Node {
                    width: px(150),
                    height: px(65),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BorderRadius::MAX,
                BackgroundColor(Color::BLACK),
                children![(
                    UiCredits,
                    Text::new(&init_credits),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ),
            (
                Node {
                    width: px(150),
                    height: px(65),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BorderRadius::MAX,
                BackgroundColor(Color::BLACK),
                children![(
                    UiBet,
                    Text::new(&init_bet),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            )
        ],
    )
}

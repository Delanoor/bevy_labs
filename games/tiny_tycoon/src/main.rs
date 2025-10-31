use std::collections::HashMap;

use bevy::prelude::*;
use core_engine::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CorePlugin))
        .insert_resource(Money(100.0))
        .insert_resource(Inventory::default())
        .insert_resource(ProductionClock(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(SavePath("saves/tycoon.ron".into()))
        .add_systems(Startup, (spawn_camera, spawn_ui, try_load_then_reconstruct))
        .add_systems(
            Update,
            (
                tick_production,
                update_ui,
                click_buy_factory,
                click_upgrade_factory,
                autosave_every_5s_dev,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveFile {
    pub money: f32,
    pub inventory: Vec<(ItemId, u32)>,
    pub factories: Vec<FactorySave>, // world snapshot
    pub last_real_secs: f64,         // offline catch-up
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

#[derive(Serialize, Deserialize, Clone)]
pub struct FactorySave {
    pub kind: FactoryKind,
    pub level: u8,
    pub pos: (f32, f32),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum FactoryKind {
    Basic,
    Advanced,
}

// =========== RUNTIME RESOURCE ===============
#[derive(Resource)]
pub struct Money(pub f32);
#[derive(Resource, Default)]
pub struct Inventory(pub HashMap<ItemId, u32>);

#[derive(Resource)]
pub struct ProductionClock(pub Timer);

#[derive(Resource)]
pub struct SavePath(std::path::PathBuf);

// =========== World Components ===============
#[derive(Component)]
pub struct Factory {
    pub kind: FactoryKind,
    pub level: u8,
}
#[derive(Component)]
pub struct Produces {
    pub item: ItemId,
    pub per_second: f32,
}

// =========== UI Components ===============

#[derive(Component)]
pub struct UiMoneyText;
#[derive(Component)]
pub struct UiInvText;
#[derive(Component)]
pub struct BtnBuyFactory(FactoryKind);
#[derive(Component)]
pub struct BtnUpgradeFactory;

fn spawn_basic_factory(mut commands: Commands, pos: Vec2, asset_server: &AssetServer) {
    let sprite = asset_server.load("cat_3.png");
    commands.spawn((
        Factory {
            kind: FactoryKind::Basic,
            level: 1,
        },
        Produces {
            item: ItemId(1),
            per_second: 1.0, // 1 item / sec
        },
        Sprite {
            image: sprite,
            custom_size: Some(Vec2::new(691.0 / 10.0, 563.0 / 10.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}

fn tick_production(
    time: Res<Time>,
    mut clock: ResMut<ProductionClock>,
    mut inv: ResMut<Inventory>,
    q: Query<&Produces>,
) {
    clock.0.tick(time.delta());
    if !clock.0.just_finished() {
        return;
    }

    for p in &q {
        *inv.0.entry(p.item).or_default() += p.per_second as u32;
    }
}

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0).with_alpha(0.92)),
        ))
        .with_children(|parent| {
            // Buy Basic Factory button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                Text::new("BUY NORMAL FACTORY"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.30, 0.0)),
                BtnBuyFactory(FactoryKind::Basic),
            ));

            // Buy Advanced Factory button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                Text::new("BUY ADVANCED FACTORY"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.30, 0.60, 0.85)),
                BtnUpgradeFactory,
            ));

            // Upgrade Factory button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                Text::new("UPGRADE FACTORY"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.30, 0.85, 0.30)),
                BtnUpgradeFactory,
            ));

            // Money display
            parent.spawn((
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                Text::new("$ 0"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                UiMoneyText,
            ));

            // Inventory display
            parent.spawn((
                Node {
                    width: Val::Px(320.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new(""),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                UiInvText,
            ));
        });
}

fn update_ui(
    money: Res<Money>,
    inv: Res<Inventory>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<UiMoneyText>>,
        Query<&mut Text, With<UiInvText>>,
    )>,
) {
    if money.is_changed() {
        if let Ok(mut t) = text_queries.p0().single_mut() {
            **t = format!("$ {:.0}", money.0);
        }
    }
    if inv.is_changed() {
        if let Ok(mut t) = text_queries.p1().single_mut() {
            let mut line = String::new();
            for (id, qty) in inv.0.iter().take(3) {
                line.push_str(&format!("I{}: {} ", id.0, qty));
            }
            **t = line;
        }
    }
}
fn click_buy_factory(
    mut commands: Commands,
    mut money: ResMut<Money>,
    asset_server: Res<AssetServer>,
    mut counter: Local<u32>,
    factories: Query<&Factory>,
    q: Query<(&Interaction, &BtnBuyFactory), Changed<Interaction>>,
) {
    // Initialize counter based on existing factories (on first run after load)
    if *counter == 0 && !factories.is_empty() {
        *counter = factories.iter().count() as u32;
    }

    for (interaction, btn) in &q {
        if *interaction == Interaction::Pressed {
            let cost = match btn.0 {
                FactoryKind::Basic => 50.0,
                FactoryKind::Advanced => 200.0,
            };

            if money.0 >= cost {
                money.0 -= cost;

                // Spawn factory at varied position using counter
                *counter += 1;
                let angle = (*counter as f32) * 0.7; // Spiral-like pattern
                let radius = (*counter as f32) * 2.0;
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;

                spawn_basic_factory(commands.reborrow(), Vec2::new(x, y), &asset_server);

                println!("Bought {:?} factory for ${}", btn.0, cost);
            }
        }
    }
}
fn click_upgrade_factory(
    mut money: ResMut<Money>,
    mut factories: Query<(&mut Factory, &mut Produces)>,
    q_btn: Query<&Interaction, (Changed<Interaction>, With<BtnUpgradeFactory>)>,
) {
    for interaction in &q_btn {
        if *interaction == Interaction::Pressed {
            // Upgrade the first factory found
            if let Some((mut factory, mut produces)) = factories.iter_mut().next() {
                let cost = (factory.level as f32) * 100.0;

                if money.0 >= cost {
                    money.0 -= cost;
                    factory.level += 1;
                    produces.per_second *= 1.5; // 50% increase per level

                    println!("Upgraded factory to level {}", factory.level);
                }
            }
        }
    }
}
fn autosave_every_5s_dev(
    time: Res<Time>,
    mut t: Local<Timer>,
    money: Res<Money>,
    inv: Res<Inventory>,
    q: Query<(&Factory, &Transform)>,
    path: Res<SavePath>,
) {
    if t.duration().is_zero() {
        *t = Timer::from_seconds(5.0, TimerMode::Repeating)
    }
    t.tick(time.delta());
    if !t.just_finished() {
        return;
    }

    use ron::ser::{PrettyConfig, to_string_pretty};
    let factories = q
        .iter()
        .map(|(f, tr)| FactorySave {
            kind: f.kind,
            level: f.level,
            pos: (tr.translation.x, tr.translation.y),
        })
        .collect::<Vec<_>>();

    let sf = SaveFile {
        money: money.0,
        inventory: inv.0.iter().map(|(k, v)| (*k, *v)).collect(),
        factories,
        last_real_secs: time.elapsed_secs_f64(),
    };

    let ron = to_string_pretty(&sf, PrettyConfig::default()).expect("serialize");
    std::fs::create_dir_all(path.0.parent().unwrap()).ok();
    std::fs::write(&path.0, ron).expect("write save");
}

// fn init_game_or_load(mut commands: Commands, asset_time: Res<Time>) {
//     commands.insert_resource(SavePath(std::path::PathBuf::from("saves/tycoon.ron")));
//     // defaults
//     commands.insert_resource(Money(100.0));
//     commands.insert_resource(Inventory::default());
// }

fn try_load_then_reconstruct(
    mut commands: Commands,
    time: Res<Time>,
    path: Res<SavePath>,
    mut money: ResMut<Money>,
    mut inv: ResMut<Inventory>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(text) = std::fs::read_to_string(&path.0) {
        if let Ok(sf) = ron::from_str::<SaveFile>(&text) {
            money.0 = sf.money;

            inv.0 = sf.inventory.into_iter().collect();

            // reconstruct factories
            for f in sf.factories {
                spawn_basic_factory(
                    commands.reborrow(),
                    Vec2::new(f.pos.0, f.pos.1),
                    &asset_server,
                );
            }

            // offline catchup
            let elapsed = (time.elapsed_secs_f64() - sf.last_real_secs).max(0.0);
            let offline_ticks = (elapsed / 1.0).floor() as u32;
            if offline_ticks > 0 {

                // TODO credits
            }
        }
    }
}

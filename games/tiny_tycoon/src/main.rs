use std::collections::HashMap;

use bevy::{asset, prelude::*};
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

fn spawn_camera() {}

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

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Component)]
pub struct UiMoneyText;
#[derive(Component)]
pub struct UiInvText;
#[derive(Component)]
pub struct BtnBuyFactory(FactoryKind);
#[derive(Component)]
pub struct BtnUpgradeFactory;

fn spawn_basic_factory(mut commands: Commands, pos: Vec2, asset_server: Res<AssetServer>) {
    let sprite = asset_server.load("cat_3.png");
    commands.spawn((
        Factory {
            kind: FactoryKind::Basic,
            level: 1,
        },
        Produces {
            item: ItemId((1)),
            per_second: 1.0, // 1 item / sec
        },
        Sprite {
            image: sprite,
            custom_size: Some(Vec2::splat(20.0)),
            image_mode: SpriteImageMode::Scale(ScalingMode::FillCenter),
            ..default()
        },
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

fn spawn_ui() {}
fn try_load_then_reconstruct() {}

fn update_ui() {}
fn click_buy_factory() {}
fn click_upgrade_factory() {}
fn autosave_every_5s_dev() {}

use std::collections::HashMap;

use bevy::prelude::*;
use core_engine::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CorePlugin))
        .add_systems(Startup, (spawn_camera))
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

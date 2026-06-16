use bevy::{asset::AssetMetaCheck, log::LogPlugin, prelude::*, window::WindowResolution};

mod key;
mod map;
mod player;

const BLOCK_SIZE: f32 = 16.0;

const GAMETITLE: &str = "いっとくフロッガー";
const WINDOW_SIZE: Vec2 = Vec2::new(BLOCK_SIZE * 39.0, BLOCK_SIZE * 29.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ittoku_frogger=debug";

const PATH_IMAGE_FROGGER: &str = "ittoku-frogger/frogger.png";
const IMAGE_FROGGER_SIZE: UVec2 = UVec2::splat(16);
const IMAGE_FROGGER_COLUMN: u32 = 7;
const IMAGE_FROGGER_ROW: u32 = 7;

const INITIAL_POSITION: Vec2 = Vec2::new(
    -WINDOW_SIZE.x / 2.0 + BLOCK_SIZE / 2.0,
    WINDOW_SIZE.y / 2.0 - BLOCK_SIZE / 2.0,
);

#[derive(Event, Deref, DerefMut)]
struct MoveEvent(Direction);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

fn main() {
    let window_size = WINDOW_SIZE.as_uvec2();
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(window_size.x, window_size.y),
            title: GAMETITLE.to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let log_plugin = LogPlugin {
        filter: LOG_FILTER.into(),
        level: bevy::log::Level::DEBUG,
        ..Default::default()
    };
    let asset_plugin = AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..Default::default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window_plugin)
                .set(log_plugin)
                .set(asset_plugin),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_plugins(key::KeyPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    info_once!("setup");

    commands.spawn(Camera2d::default());
}

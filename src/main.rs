use bevy::{
    prelude::*,
    log::LogPlugin,
    asset::AssetMetaCheck,
};

mod key;
mod player;
mod map;

const GAMETITLE: &str = "いっとくフロッガー";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ittoku_frogger=debug";
const PATH_IMAGE_PLAYER: &str = "ittoku-frogger/player.png";
const PATH_IMAGE_MAP: &str = "ittoku-frogger/map.png";

#[derive(Event, Deref, DerefMut)]
struct MoveEvent(Direction);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                filter: LOG_FILTER.into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
        )
        .add_event::<MoveEvent>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_plugins(key::KeyPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(map::MapPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    info_once!("setup");

    commands.spawn(Camera2d::default());
}

use bevy::{
    prelude::*,
    log::LogPlugin,
    asset::AssetMetaCheck,
};

const GAMETITLE: &str = "いっとくフロッガー";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ittoku_frogger=debug";
const PATH_IMAGE_PLAYER: &str = "ittoku-frogger/player.png";

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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    info_once!("setup");
    debug!("spawn camera");
    commands.spawn(Camera2d::default());
}

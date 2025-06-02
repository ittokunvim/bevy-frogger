use bevy::prelude::*;

use crate::PATH_IMAGE_MAP;

const MAP_POSITION: Vec3 = Vec3::new(0.0, 0.0, -99.0);

fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("map_setup");

    let map = asset_server.load(PATH_IMAGE_MAP);
    commands.spawn((
        Sprite::from_image(map),
        Transform::from_translation(MAP_POSITION),
    ));
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, map_setup)
        ;
    }
}

use bevy::prelude::*;

use crate::{
    BLOCK_SIZE, IMAGE_FROGGER_COLUMN, IMAGE_FROGGER_ROW, IMAGE_FROGGER_SIZE, INITIAL_POSITION,
    PATH_IMAGE_FROGGER,
};

// マップ背景タイプの定義
// 0: 土地（黒色）
// 1: 水（青色）
// 2: 土地（紫色）
// 3: 芝生（緑色）
const MAP_DATA_BACKGROUND: [[usize; 15]; 17] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
    [3, 1, 3, 3, 1, 3, 3, 1, 3, 3, 1, 3, 3, 1, 3],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

// マップ背景を表すコンポーネント
#[derive(Component, Debug)]
struct MapBackground;

/// マップの背景を描画する
fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info_once!("map_setup");

    let texture = asset_server.load(PATH_IMAGE_FROGGER);
    let layout = TextureAtlasLayout::from_grid(
        IMAGE_FROGGER_SIZE,
        IMAGE_FROGGER_COLUMN,
        IMAGE_FROGGER_ROW,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // マップの背景を描画する
    for (column, i) in MAP_DATA_BACKGROUND.iter().enumerate() {
        for (row, j) in i.iter().enumerate() {
            let sprite = match j {
                0 => Sprite::from_color(Color::srgb(0.1, 0.1, 0.1), Vec2::splat(BLOCK_SIZE)),
                1 => Sprite::from_color(Color::srgb(0.1, 0.1, 0.9), Vec2::splat(BLOCK_SIZE)),
                2 => Sprite::from_atlas_image(
                    texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 36,
                    },
                ),
                3 => Sprite::from_atlas_image(
                    texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 35,
                    },
                ),
                _ => Sprite::default(),
            };
            let translation = Vec3::new(
                INITIAL_POSITION.x + (BLOCK_SIZE * row as f32),
                INITIAL_POSITION.y - (BLOCK_SIZE * column as f32),
                -99.0,
            );
            commands.spawn((
                sprite,
                Transform::from_translation(translation),
                MapBackground,
            ));
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, map_setup);
    }
}

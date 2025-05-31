use std::time::Duration;
use bevy::prelude::*;

use crate::PATH_IMAGE_PLAYER;

const IMAGE_SIZE: UVec2 = UVec2::splat(18);
const IMAGE_COLUMN: u32 = 8;
const IMAGE_ROW: u32 = 4;

/// プレイヤーが操作をするコンポーネント
/// - first_sprite_index: アニメーションの最初のインデックス
/// - last_sprite_index: アニメーションの最後のインデックス
/// - frame_timer: アニメーションを行う表示速度
#[derive(Component, Debug)]
struct Player {
    first_sprite_index: usize,
    last_sprite_index: usize,
    frame_timer: Timer,
}

impl Player {
    const FPS: u8 = 4;

    fn new() -> Self {
        let secs = 1.0 / Self::FPS as f32;
        let frame_timer = Timer::new(Duration::from_secs_f32(secs), TimerMode::Repeating);

        Self {
            first_sprite_index: 0,
            last_sprite_index: 3,
            frame_timer,
        }
    }
}

/// プレイヤーのセットアップを行う関数
fn player_setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    info_once!("player_setup");

    let texture = asset_server.load(PATH_IMAGE_PLAYER);
    let layout = TextureAtlasLayout::from_grid(IMAGE_SIZE, IMAGE_COLUMN, IMAGE_ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::splat(2.0)),
        Player::new(),
    ));
}

/// プレイヤーをアニメーションするための関数
fn player_animate(
    mut query: Query<(&mut Player, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    info_once!("player_animate");

    for (mut player, mut sprite) in &mut query {
        player.frame_timer.tick(time.delta());

        if player.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == player.last_sprite_index {
                    atlas.index = player.first_sprite_index;
                } else {
                    atlas.index += 1;
                };
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, player_setup)
            .add_systems(Update, player_animate)
        ;
    }
}

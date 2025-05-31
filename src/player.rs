use std::{f32, time::Duration};
use bevy::prelude::*;

use crate::{Direction, MoveEvent, PATH_IMAGE_PLAYER};

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

#[derive(Component, Debug, Deref, DerefMut)]
struct Velocity(Vec2);

impl Player {
    const FPS: u8 = 4;
    const SPEED: f32 = 32.0;
    const DISTANCE: f32 = 32.0;
    const INDICES_LEFT: (usize, usize) = (8, 11);
    const INDICES_RIGHT: (usize, usize) = (16, 19);
    const INDICES_TOP: (usize, usize) = (24, 27);
    const INDICES_BOTTOM: (usize, usize) = (0, 3);

    fn new() -> Self {
        let first_sprite_index = Self::INDICES_BOTTOM.0;
        let last_sprite_index = Self::INDICES_BOTTOM.1;
        let secs = 1.0 / Self::FPS as f32;
        let frame_timer = Timer::new(Duration::from_secs_f32(secs), TimerMode::Repeating);

        Self {
            first_sprite_index,
            last_sprite_index,
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
        Velocity(Vec2::ZERO),
    ));
}

/// プレイヤーをアニメーションするための関数
fn player_animation(
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

/// イベントを受け取り、プレイヤーのアニメーションの振る舞いを決める関数
fn player_change_animation(
    mut events: EventReader<MoveEvent>,
    mut query: Query<(&mut Player, &mut Sprite), With<Player>>,
) {
    info_once!("player_change_animation");

    // イベントを受け取ったら処理を実行
    for event in events.read() {
        // プレイヤーの値を取得。プレイヤーがなければ処理を抜ける
        let Ok((mut player, mut sprite)) = query.get_single_mut() else {
            return;
        };

        // アニメーションのインデックスを更新
        match **event {
            Direction::Left => {
                player.first_sprite_index = Player::INDICES_LEFT.0;
                player.last_sprite_index = Player::INDICES_LEFT.1;
            }
            Direction::Right => {
                player.first_sprite_index = Player::INDICES_RIGHT.0;
                player.last_sprite_index = Player::INDICES_RIGHT.1;
            }
            Direction::Top => {
                player.first_sprite_index = Player::INDICES_TOP.0;
                player.last_sprite_index = Player::INDICES_TOP.1;
            }
            Direction::Bottom => {
                player.first_sprite_index = Player::INDICES_BOTTOM.0;
                player.last_sprite_index = Player::INDICES_BOTTOM.1;
            }
        }

        // スプライトのインデックスを更新
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = player.first_sprite_index;
        };
    }
}

/// プレイヤーの移動を管理する関数
fn player_movement(
    mut events: EventReader<MoveEvent>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    info_once!("player_movement");

    // プレイヤーの値を取得。プレイヤーがなければ処理を抜ける
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let x = transform.translation.x;
    let y = transform.translation.y;
    let dist = Player::DISTANCE;

    // x座標がDISTANCEの倍数付近なら丸めて停止
    if (x % dist).abs() < 1.0 {
        velocity.x = 0.0;
        transform.translation.x = (x / dist).round() * dist;
    }
    // y座標がDISTANCEの倍数付近なら丸めて停止
    if (y % dist).abs() < 1.0 {
        velocity.y = 0.0;
        transform.translation.y = (y / dist).round() * dist;
    }

    for event in events.read() {
        match **event {
            Direction::Left => {
                transform.translation.x -= 1.0;
                velocity.x = -Player::SPEED;
            }
            Direction::Right => {
                transform.translation.x += 1.0;
                velocity.x = Player::SPEED;
            }
            Direction::Top => {
                transform.translation.y += 1.0;
                velocity.y = Player::SPEED;
            }
            Direction::Bottom => {
                transform.translation.y -= 1.0;
                velocity.y = -Player::SPEED;
            }
        }
    }
}

/// 速度に応じてコンポーネントを移動する関数
fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), With<Velocity>>,
    time_step: Res<Time<Fixed>>,
) {
    info_once!("apply_velocity");

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, player_setup)
            .add_systems(Update, (
                player_animation,
                player_change_animation,
                player_movement,
                apply_velocity,
            ))
        ;
    }
}

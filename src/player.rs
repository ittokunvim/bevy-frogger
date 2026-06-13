use bevy::prelude::*;
use std::{f32, time::Duration};

use crate::{
    Direction, MoveEvent, IMAGE_FROGGER_COLUMN, IMAGE_FROGGER_ROW, IMAGE_FROGGER_SIZE,
    PATH_IMAGE_FROGGER,
};

/// プレイヤーが操作をするコンポーネント
/// - first_sprite_index: アニメーションの最初のインデックス
#[derive(Component, Debug)]
struct Player {
    /// - last_sprite_index: アニメーションの最後のインデックス
    /// - frame_timer: アニメーションを行う表示速度
    first_sprite_index: usize,
    last_sprite_index: usize,
    frame_timer: Timer,
}

/// 速度を操作するコンポーネント
/// - 0: xyの値で向きと速度を定義する
#[derive(Component, Debug, Deref, DerefMut)]
struct Velocity(Vec2);

impl Player {
    const FPS: u8 = 2;
    const SPEED: f32 = 16.0;
    const DISTANCE: f32 = 16.0;
    const IDLE_INDICES: (usize, usize) = (0, 0);
    const MOVE_INDICES: (usize, usize) = (1, 2);

    // プレイヤーをセットアップする
    fn new() -> Self {
        let first_sprite_index = Self::IDLE_INDICES.0;
        let last_sprite_index = Self::IDLE_INDICES.1;
        let secs = 1.0 / Self::FPS as f32;
        let frame_timer = Timer::new(Duration::from_secs_f32(secs), TimerMode::Repeating);

        Self {
            first_sprite_index,
            last_sprite_index,
            frame_timer,
        }
    }

    // プレイヤーのアニメーションを変更する
    fn set_animation_indices(&mut self, indices: (usize, usize)) {
        self.first_sprite_index = indices.0;
        self.last_sprite_index = indices.1;
    }
}

/// プレイヤーのセットアップを行う関数
fn player_setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    info_once!("player_setup");

    let texture = asset_server.load(PATH_IMAGE_FROGGER);
    let layout = TextureAtlasLayout::from_grid(
        IMAGE_FROGGER_SIZE,
        IMAGE_FROGGER_COLUMN,
        IMAGE_FROGGER_ROW,
        None,
        None,
    );
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
fn player_animation(mut query: Query<(&mut Player, &mut Sprite), With<Player>>, time: Res<Time>) {
    info_once!("player_animation");

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

/// プレイヤーの移動を止める関数
fn player_stop(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Player, &mut Sprite), With<Player>>,
) -> Result {
    info_once!("player_stop");

    // プレイヤーの値を取得。プレイヤーがなければ処理を抜ける
    let (mut transform, mut velocity, mut player, mut sprite) = query.single_mut()?;
    let dist = Player::DISTANCE;

    // x座標がDISTANCEの倍数付近なら丸めて停止
    if (transform.translation.x % dist).abs() < 1.0 {
        velocity.x = 0.0;
        transform.translation.x = (transform.translation.x / dist).round() * dist;
    }
    // y座標がDISTANCEの倍数付近なら丸めて停止
    if (transform.translation.y % dist).abs() < 1.0 {
        velocity.y = 0.0;
        transform.translation.y = (transform.translation.y / dist).round() * dist;
    }

    // プレイヤーが止まったら向きを戻す
    if velocity.x == 0.0 && velocity.y == 0.0 {
        transform.rotation = Quat::from_rotation_z(0.0);
        player.set_animation_indices(Player::IDLE_INDICES);
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = player.first_sprite_index;
        }
    }

    Ok(())
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

/// プレイヤーの移動を管理する関数
fn player_movement(
    moved: On<MoveEvent>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut Player, &mut Sprite), With<Player>>,
) -> Result {
    info_once!("player_movement");

    // プレイヤーの値を取得。プレイヤーがなければ処理を抜ける
    let (mut transform, mut velocity, mut player, mut sprite) = query.single_mut()?;

    // プレイヤーのアニメーションを開始する
    player.set_animation_indices(Player::MOVE_INDICES);
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = player.first_sprite_index;
    }

    // プレイヤーを向きに応じて動かす
    match **moved {
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

    Ok(())
}

/// イベントを受け取り、プレイヤーの回転を行う関数
fn player_rotation(moved: On<MoveEvent>, mut query: Query<&mut Transform, With<Player>>) -> Result {
    info_once!("player_rotation");

    // プレイヤーの値を取得。プレイヤーがなければ処理を抜ける
    let mut transform = query.single_mut()?;

    // プレイヤーの向きを変更
    match **moved {
        Direction::Left => {
            transform.rotation = Quat::from_rotation_z(1.6);
        }
        Direction::Right => {
            transform.rotation = Quat::from_rotation_z(-1.6);
        }
        Direction::Top => {
            transform.rotation = Quat::from_rotation_z(0.0);
        }
        Direction::Bottom => {
            transform.rotation = Quat::from_rotation_z(3.15);
        }
    }

    Ok(())
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, (player_animation, player_stop, apply_velocity))
            .add_observer(player_movement)
            .add_observer(player_rotation);
    }
}

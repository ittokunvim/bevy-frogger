use bevy::prelude::*;

use crate::{Direction, MoveEvent};

const KEY_PLAYER_LEFT: KeyCode = KeyCode::ArrowLeft;
const KEY_PLAYER_RIGHT: KeyCode = KeyCode::ArrowRight;
const KEY_PLAYER_TOP: KeyCode = KeyCode::ArrowUp;
const KEY_PLAYER_BOTTOM: KeyCode = KeyCode::ArrowDown;

fn key_player_move_left(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    info_once!("key_player_move_left");

    if keyboard_input.just_pressed(KEY_PLAYER_LEFT) {
        commands.trigger(MoveEvent(Direction::Left));
    }
}

fn key_player_move_right(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    info_once!("key_player_move_right");

    if keyboard_input.just_pressed(KEY_PLAYER_RIGHT) {
        commands.trigger(MoveEvent(Direction::Right));
    }
}

fn key_player_move_top(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    info_once!("key_player_move_top");

    if keyboard_input.just_pressed(KEY_PLAYER_TOP) {
        commands.trigger(MoveEvent(Direction::Top));
    }
}

fn key_player_move_bottom(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    info_once!("key_player_move_bottom");

    if keyboard_input.just_pressed(KEY_PLAYER_BOTTOM) {
        commands.trigger(MoveEvent(Direction::Bottom));
    }
}

pub struct KeyPlugin;

impl Plugin for KeyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                key_player_move_left,
                key_player_move_right,
                key_player_move_top,
                key_player_move_bottom,
            ),
        );
    }
}

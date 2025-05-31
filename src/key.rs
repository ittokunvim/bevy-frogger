use bevy::prelude::*;

use crate::{Direction, MoveEvent};

const KEY_PLAYER_LEFT: KeyCode = KeyCode::ArrowLeft;
const KEY_PLAYER_RIGHT: KeyCode = KeyCode::ArrowRight;
const KEY_PLAYER_TOP: KeyCode = KeyCode::ArrowUp;
const KEY_PLAYER_BOTTOM: KeyCode = KeyCode::ArrowDown;

fn key_player_move_left(
    mut events: EventWriter<MoveEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_player_move_left");

    if keyboard_input.just_pressed(KEY_PLAYER_LEFT) {
        events.send(MoveEvent(Direction::Left));
    }
}

fn key_player_move_right(
    mut events: EventWriter<MoveEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_player_move_right");

    if keyboard_input.just_pressed(KEY_PLAYER_RIGHT) {
        events.send(MoveEvent(Direction::Right));
    }
}

fn key_player_move_top(
    mut events: EventWriter<MoveEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_player_move_top");

    if keyboard_input.just_pressed(KEY_PLAYER_TOP) {
        events.send(MoveEvent(Direction::Top));
    }
}

fn key_player_move_bottom(
    mut events: EventWriter<MoveEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_player_move_bottom");

    if keyboard_input.just_pressed(KEY_PLAYER_BOTTOM) {
        events.send(MoveEvent(Direction::Bottom));
    }
}

pub struct KeyPlugin;

impl Plugin for KeyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
                key_player_move_left,
                key_player_move_right,
                key_player_move_top,
                key_player_move_bottom,
            ))
        ;
    }
}

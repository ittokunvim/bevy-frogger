use bevy::{input::keyboard::KeyboardInput, prelude::*};
use std::f32::consts::PI;

const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);
const PATH_MODELS_PLAYER: &str = "models/gekota.glb";
const PATH_MODELS_TILE: &str = "models/tile.glb";

const BOARD_SIZE_I: usize = 12;
const BOARD_SIZE_J: usize = 8;

const CAMERA_SPEED: f32 = 2.0;
const CAMERA_DISTANCE: Vec3 = Vec3::new(-2.8, 3.0, 3.5);

const PLAYER_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, BOARD_SIZE_J as f32 / 2.0);

const OBSTACLE_SIZE: f32 = 0.8;

const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const OBSTACLE_COLOR: Color = Color::srgb(0.8, 0.7, 0.6);
const PRESSANYKEY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::InGame)))
        .add_systems(
            Update,
            check_for_collision.run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, move_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, move_obstacle.run_if(in_state(AppState::InGame)))
        .add_systems(Update, focus_camera.run_if(in_state(AppState::InGame)))
        .add_systems(Update, goal_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
        .run();
}

#[derive(Component)]
struct Camera {
    looking_at: Vec3,
}

#[derive(Component)]
struct Player {
    i: f32,
    j: f32,
    move_cooldown: Timer,
}

#[derive(Component)]
struct Obstacle {
    i: f32,
    j: f32,
}

#[derive(Component)]
struct PressAnyKey;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct ScoreSpan;

#[derive(Resource, Deref, DerefMut)]
struct Score(isize);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    let translation = PLAYER_INITIAL_POSITION + CAMERA_DISTANCE;
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(translation)
            .looking_at(Vec3::from(PLAYER_INITIAL_POSITION), Vec3::Y),
        Camera {
            looking_at: Vec3::from(PLAYER_INITIAL_POSITION),
        },
    ));

    // Light
    let translation = Vec3::new(4.0, 10.0, 4.0);
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            range: 30.0,
            ..Default::default()
        },
        Transform::from_translation(translation),
    ));

    // Board
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset(PATH_MODELS_TILE));
    for i in 0..BOARD_SIZE_I {
        for j in 0..BOARD_SIZE_J {
            let translation = Vec3::new(i as f32, -0.2, j as f32);
            commands.spawn((
                SceneRoot(model.clone()),
                Transform::from_translation(translation),
            ));
        }
    }

    // Player
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset(PATH_MODELS_PLAYER));
    commands.spawn((
        SceneRoot(model),
        Transform {
            translation: PLAYER_INITIAL_POSITION,
            rotation: Quat::from_rotation_y(PI / 2.0),
            ..Default::default()
        },
        Player {
            i: PLAYER_INITIAL_POSITION.x,
            j: PLAYER_INITIAL_POSITION.z,
            move_cooldown: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));

    // Obstacles
    let shape = Cuboid::new(OBSTACLE_SIZE, OBSTACLE_SIZE, OBSTACLE_SIZE);
    for i in 1..BOARD_SIZE_I - 1 {
        if i % 2 == 0 {
            continue;
        }

        let translation = Vec3::new(i as f32, OBSTACLE_SIZE / 2.0, 0.0);
        commands.spawn((
            Mesh3d(meshes.add(shape)),
            MeshMaterial3d(materials.add(OBSTACLE_COLOR)),
            Transform::from_translation(translation),
            Obstacle {
                i: translation.x,
                j: translation.z,
            },
            Velocity(Vec3::new(0.0, 0.0, i as f32)),
        ));
    }

    // Scoreboard
    commands.spawn((
        Text::new("Score: "),
        TextFont::from_font_size(20.0),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..Default::default()
        },
    ))
    .with_child((
        TextSpan::default(),
        TextFont::from_font_size(20.0),
        ScoreSpan,
    ));

    // Press any key
    commands.spawn((
        Text::new("Press Any Key ...".to_string()),
        TextFont::from_font_size(20.0),
        TextColor(PRESSANYKEY_COLOR),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.0),
            bottom: Val::Px(5.0),
            ..Default::default()
        },
        PressAnyKey,
    ));
}

fn apply_velocity(
    mut query: Query<(&mut Obstacle, &mut Transform, &Velocity)>,
    time_step: Res<Time<Fixed>>
) {
    for (mut obstacle, mut transform, velocity) in &mut query {
        obstacle.j = transform.translation.z;
        transform.translation.z += velocity.z * time_step.delta().as_secs_f32();
    }
}

fn check_for_collision(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    obstacle_query: Query<&Obstacle, With<Obstacle>>,
    mut score: ResMut<Score>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let player_half_size_z = player_transform.scale.z / 2.0;
    let player_j = (
        player.j as f32 - player_half_size_z,
        player.j as f32 + player_half_size_z,
    );

    for obstacle in &obstacle_query {
        let obstacle_half_size_z = OBSTACLE_SIZE / 2.0;
        let obstacle_j = (
            obstacle.j as f32 - obstacle_half_size_z,
            obstacle.j as f32 + obstacle_half_size_z,
        );

        if player_j.0 < obstacle_j.1 && player_j.1 > obstacle_j.0 && player.i == obstacle.i {
            **score -= 1;
            player.i = PLAYER_INITIAL_POSITION.x;
            player.j = PLAYER_INITIAL_POSITION.z;
            player_transform.translation = PLAYER_INITIAL_POSITION;
        }
    }
}

fn update_scoreboard(
    mut query: Query<&mut TextSpan, With<ScoreSpan>>,
    score: Res<Score>,
) {
    let Ok(mut span) = query.get_single_mut() else {
        return;
    };
    **span = score.0.to_string();
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    if player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut rotation = 0.0;

        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            if player.i < BOARD_SIZE_I as f32 - 1.0 {
                player.i += 1.0;
            }
            rotation = PI / 2.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            if player.i > 0.0 {
                player.i -= 1.0;
            }
            rotation = -PI / 2.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            if player.j < BOARD_SIZE_J as f32 - 1.0 {
                player.j += 1.0;
            }
            rotation = 0.0;
            moved = true;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            if player.j > 0.0 {
                player.j -= 1.0;
            }
            rotation = PI;
            moved = true;
        }

        if moved {
            player.move_cooldown.reset();
            player_transform.translation = Vec3::new(player.i as f32, 0.0, player.j as f32);
            player_transform.rotation = Quat::from_rotation_y(rotation);
        }
    }
}

fn move_obstacle(mut obstacle_query: Query<(&Transform, &mut Velocity), With<Obstacle>>) {
    for (obstacle_transform, mut obstacle_velocity) in &mut obstacle_query {
        let left_board_collision = 0.0 > obstacle_transform.translation.z;
        let right_board_collision =
            (BOARD_SIZE_J as f32) < obstacle_transform.translation.z + OBSTACLE_SIZE;

        if left_board_collision || right_board_collision {
            obstacle_velocity.z = -obstacle_velocity.z;
        }
    }
}

fn focus_camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut Camera, &mut Transform), (With<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let (mut camera, mut camera_transform) = camera_query.single_mut();
    let player_transform = player_query.single();
    let motion_time = CAMERA_SPEED * time.delta_secs();

    // move camera position
    let mut camera_motion =
        player_transform.translation + CAMERA_DISTANCE - camera_transform.translation;
    if camera_motion.length() > 0.2 {
        camera_motion *= motion_time;
        camera_transform.translation += camera_motion;
    }

    // move camera looking position
    let mut camera_motion = player_transform.translation - camera.looking_at;
    if camera_motion.length() > 0.2 {
        camera_motion *= motion_time;
        camera.looking_at += camera_motion;
    }
    *camera_transform = camera_transform.looking_at(camera.looking_at, Vec3::Y);
}

fn goal_player(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    mut score: ResMut<Score>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    if player.i >= BOARD_SIZE_I as f32 - 1.0 {
        **score += 1;
        player.i = PLAYER_INITIAL_POSITION.x;
        player.j = PLAYER_INITIAL_POSITION.z;
        player_transform.translation = PLAYER_INITIAL_POSITION;
    }
}

fn press_any_key(
    mut keyboard_event: EventReader<KeyboardInput>,
    pressanykey_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<ButtonInput<KeyCode>>,
) {
    for _event in keyboard_event.read() {
        let pressanykey_entity = pressanykey_query.single();
        commands.entity(pressanykey_entity).despawn();

        *now_state = State::new(AppState::InGame);
        inkey.reset_all();
    }
}

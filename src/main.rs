use bevy::{prelude::*, window::PrimaryWindow};

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Resource, Default)]
struct MouseWorldCoords(Vec2);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct MovementTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .init_resource::<MouseWorldCoords>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (cursor_moved, player_rotation).chain(),
                (mouse_click, update_scoreboard).chain(),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player_ship.png"),
            ..default()
        },
        Player,
    ));

    // Scoreboard
    commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}

fn cursor_moved(
    mut mouse_world_coords: ResMut<MouseWorldCoords>,
    mut cursor_event_reader: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();

    for event in cursor_event_reader.read() {
        // TODO: Should this be ignoring everything but the last event? Is there a better way to update this?
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, event.position)
        {
            mouse_world_coords.0 = world_position;
        }
    }
}

fn player_rotation(
    mouse_world_coords: Res<MouseWorldCoords>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_transform_query.single_mut();

    let to_mouse_world_coords =
        (mouse_world_coords.0 - player_transform.translation.xy()).normalize_or_zero();

    player_transform.rotation = Quat::from_rotation_arc(Vec3::Y, to_mouse_world_coords.extend(0.0));
}

fn mouse_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut scoreboard: ResMut<Scoreboard>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        if let Some(world_position) = window_query
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            info!(
                "Cursor clicked in the window at world position {:?}",
                world_position
            );

            let mut player_transform = player_transform_query.single_mut();
            player_transform.translation.x = world_position.x;
            player_transform.translation.y = world_position.y;

            scoreboard.score += 1;
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

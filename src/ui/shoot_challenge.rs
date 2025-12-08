use bevy::{
    color::palettes::tailwind::{AMBER_300, FUCHSIA_800, GRAY_500, INDIGO_700, RED_600},
    prelude::*,
};

use crate::state::state::AppState;

pub struct ShootChallengePlugin;
impl Plugin for ShootChallengePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AimChallengeState>()
            .add_systems(
                OnEnter(AppState::Aim),
                (spawn_shoot_challenge, reset_shoot_challenge_state),
            )
            .add_systems(OnExit(AppState::Aim), despawn_shoot_challenge)
            .add_systems(
                Update,
                (
                    shoot_challenge_input_handler,
                    update_position_cursor_marker,
                    update_power_cursor_marker,
                    update_power_indicator,
                    update_precision_cursor_marker,
                    progress_cursor,
                )
                    .run_if(in_state(AppState::Aim)),
            );
    }
}

#[derive(Component)]
struct ShootChallengeBars;

// the positions use a not so straight forward scale
// furthest left is position 1.0
// furthest right is position -0.2
// starting position is 0.0
// during game the curser moves towards left (up), allowing the user to press space to set the power marker
// if the dont, they miss
// after reaching 1 the cursor moves left
// the user needs to try to hit as close to 0 for full precision
// there is a certain grace area (based on the club) -> if they don't hit the cursor in this area
// the shot misses e.g. uncontrolled shot
#[derive(Resource)]
struct AimChallengeResource {
    cursor_pos: f32,
    power_marker: Option<f32>,
    precision_marker: Option<f32>,
}

#[derive(Component)]
struct OriginMarker;
#[derive(Component)]
struct CursorMarker;
#[derive(Component)]
struct PowerMarker;
#[derive(Component)]
struct PrecisionMarker;
#[derive(Component)]
struct PowerIndicator;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AimChallengeState {
    #[default]
    Idle,
    Forward,
    Reverse,
    Finalized,
}

fn spawn_shoot_challenge(mut commands: Commands) {
    info!("creating shoot challenge ui");
    commands
        .spawn((
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::End,
                padding: UiRect {
                    bottom: px(50),
                    ..default()
                },
                min_width: px(100),
                min_height: px(10),
                ..default()
            },
            ShootChallengeBars,
        ))
        .with_children(|builder| {
            spawn_background(builder);

            spawn_precision_indicator(builder);
            spawn_gradient(builder);
            spawn_marker(builder, OriginMarker);
            spawn_marker(builder, CursorMarker);
            spawn_marker(builder, PowerMarker);
            spawn_marker(builder, PrecisionMarker);
        });

    info!("inserting shoot challenge resources");
    commands.insert_resource(AimChallengeResource {
        cursor_pos: 0.0,
        power_marker: None,
        precision_marker: None,
    });
}

fn spawn_background(builder: &mut ChildSpawnerCommands) {
    builder
        .spawn((
            Node {
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: px(-5),
                    ..default()
                },
                column_gap: px(8),
                ..default()
            },
            ShootChallengeBars,
        ))
        .with_children(|builder| {
            // spanw 30 vertical bars
            for i in 0..30 {
                let height = match i {
                    0 => px(30),
                    i if i % 5 == 0 => px(10),
                    _ => px(20),
                };
                builder.spawn((
                    Node {
                        height,
                        width: px(10),
                        ..default()
                    },
                    BackgroundColor(bevy::prelude::Color::Srgba(AMBER_300)),
                ));
            }
        });
}

fn _res_unit_to_perc(input: f32) -> Val {
    percent(100.0 - ((input + 0.2) / 1.2) * 100.0)
}

fn update_position_cursor_marker(
    res: Res<AimChallengeResource>,
    mut marker: Single<&mut Node, With<CursorMarker>>,
) {
    marker.left = _res_unit_to_perc(res.cursor_pos);
}

fn update_power_cursor_marker(
    res: Res<AimChallengeResource>,
    mut marker: Single<&mut Node, With<PowerMarker>>,
) {
    if let Some(power_marker) = res.power_marker {
        marker.left = _res_unit_to_perc(power_marker);
        marker.display = Display::Block;
    } else {
        marker.display = Display::None;
    }
}

fn update_power_indicator(
    res: Res<AimChallengeResource>,
    mut indicator: Single<&mut Node, With<PowerIndicator>>,
) {
    if let Some(power_marker) = res.power_marker {
        indicator.left = _res_unit_to_perc(power_marker);
    } else {
        indicator.left = _res_unit_to_perc(res.cursor_pos);
    }
}

fn update_precision_cursor_marker(
    res: Res<AimChallengeResource>,
    mut marker: Single<&mut Node, With<PrecisionMarker>>,
) {
    if let Some(precision_marker) = res.precision_marker {
        marker.left = _res_unit_to_perc(precision_marker);
        marker.display = Display::Block;
    } else {
        marker.display = Display::None;
    }
}

fn spawn_marker<T: Bundle>(builder: &mut ChildSpawnerCommands, marker_type: T) {
    builder.spawn((
        Node {
            width: px(10),
            margin: UiRect {
                left: px(-5),
                ..default()
            },
            height: px(30),
            position_type: PositionType::Absolute,
            left: _res_unit_to_perc(0.0),
            align_self: AlignSelf::Center,
            ..default()
        },
        BackgroundColor(bevy::prelude::Color::Srgba(GRAY_500)),
        marker_type,
    ));
}

fn spawn_gradient(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Node {
            width: auto(),
            height: px(20),
            position_type: PositionType::Absolute,
            left: _res_unit_to_perc(0.0),
            right: _res_unit_to_perc(0.8),
            align_self: AlignSelf::Center,
            ..default()
        },
        BackgroundGradient(vec![
            LinearGradient {
                angle: 90.0,
                stops: vec![
                    ColorStop::new(INDIGO_700, Val::Percent(0.0)),
                    ColorStop::new(FUCHSIA_800, Val::Percent(100.0)),
                ],
                ..default()
            }
            .into(),
        ]),
        PowerIndicator,
    ));
}

fn spawn_precision_indicator(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        Node {
            width: auto(),
            height: px(4),
            position_type: PositionType::Absolute,
            left: _res_unit_to_perc(0.1),
            right: _res_unit_to_perc(0.9),
            margin: UiRect {
                bottom: px(-8),
                ..default()
            },
            align_self: AlignSelf::End,
            ..default()
        },
        BackgroundColor(bevy::prelude::Color::Srgba(RED_600)),
    ));
}

// draw gradient between 0.0 and power_marker if exists
// if power_marker doesn't exist between 0.0 and cursor_pos as long as state is

fn despawn_shoot_challenge(
    mut commands: Commands,
    ui_query: Query<Entity, With<ShootChallengeBars>>,
) {
    for entity in &ui_query {
        info!("removing shoot challenge ui");
        commands.entity(entity).despawn();
    }
    info!("removing shoot challenge resources");
    commands.remove_resource::<AimChallengeResource>();
}

fn reset_shoot_challenge_state(mut next_state: ResMut<NextState<AimChallengeState>>) {
    next_state.set(AimChallengeState::Idle);
}

fn progress_cursor(
    mut data: ResMut<AimChallengeResource>,
    state: Res<State<AimChallengeState>>,
    mut next_state: ResMut<NextState<AimChallengeState>>,
) {
    if *state.get() == AimChallengeState::Forward {
        data.cursor_pos += 0.015;
    }
    if *state.get() == AimChallengeState::Reverse {
        data.cursor_pos -= 0.015;
    }

    if *state.get() == AimChallengeState::Forward && data.cursor_pos >= 1.0 {
        next_state.set(AimChallengeState::Reverse);
    }
    if *state.get() == AimChallengeState::Reverse
        && data.power_marker != None
        && data.cursor_pos <= -0.2
    {
        info!("shoot challenge is finalized");
        next_state.set(AimChallengeState::Finalized)
    }
    if *state.get() == AimChallengeState::Reverse
        && data.power_marker == None
        && data.cursor_pos <= 0.0
    {
        info!("shoot challenge is reset");
        next_state.set(AimChallengeState::Idle)
    }
}

fn shoot_challenge_input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut data: ResMut<AimChallengeResource>,
    state: Res<State<AimChallengeState>>,
    mut next_state: ResMut<NextState<AimChallengeState>>,
) {
    if *state.get() == AimChallengeState::Idle && keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AimChallengeState::Forward);
    } else if (*state.get() == AimChallengeState::Forward
        || *state.get() == AimChallengeState::Reverse)
        && keyboard_input.just_pressed(KeyCode::Space)
        && data.power_marker == None
    {
        info!("setting power marker to {}", data.cursor_pos);
        data.power_marker = Some(data.cursor_pos);
    } else if *state.get() == AimChallengeState::Reverse
        && data.power_marker != None
        && data.precision_marker == None
        && keyboard_input.just_pressed(KeyCode::Space)
    {
        info!("setting precision marker to {}", data.cursor_pos);
        data.precision_marker = Some(data.cursor_pos);

        next_state.set(AimChallengeState::Finalized);
    }
}

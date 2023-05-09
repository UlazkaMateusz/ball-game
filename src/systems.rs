use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn transision_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Enetered Game state");
        }
    }
}

pub fn transision_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Enetered menu state");
        }
    }
}

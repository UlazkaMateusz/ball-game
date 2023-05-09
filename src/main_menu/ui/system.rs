use bevy::{app::AppExit, prelude::*};

use crate::AppState;

use super::{
    components::{MainMenu, PlayButton, QuitButton},
    layout::build_main_menu,
    styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn interact_with_play_button(
    mut play_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = play_button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut quit_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = quit_button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

use bevy::prelude::*;

use super::components::{MainMenu, PlayButton, QuitButton};

use super::styles::*;

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..Default::default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            spawn_title(parent, asset_server);

            // Play button
            spawn_play_button(parent, asset_server);

            // Quit button
            spawn_quit_button(parent, asset_server);
        })
        .id();

    main_menu_entity
}

fn spawn_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: TITLE_STYLE,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: TITLE_IMAGE_STYLE,
                image: asset_server.load("sprites/ball_blue_large.png").into(),
                ..Default::default()
            });

            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Bevy Ball Game",
                        get_title_style(asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn(ImageBundle {
                style: TITLE_IMAGE_STYLE,
                image: asset_server.load("sprites/ball_red_large.png").into(),
                ..Default::default()
            });
        });
}

fn spawn_play_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            ButtonBundle {
                style: NORMAL_BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            },
            PlayButton {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Play", get_button_style(asset_server))],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn spawn_quit_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            ButtonBundle {
                style: NORMAL_BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            },
            QuitButton {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Quit", get_button_style(asset_server))],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

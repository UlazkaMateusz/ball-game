use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::{exit_game, spawn_camera, transision_to_game_state, transision_to_main_menu_state};

pub mod game;
pub mod main_menu;
pub mod systems;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(transision_to_game_state)
        .add_system(transision_to_main_menu_state)
        .run();
}

use bevy::prelude::*;

use self::ui::UIPlugin;

pub mod ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UIPlugin);
    }
}

pub mod enemy;
use bevy::prelude::*;

use crate::AppState;

use self::{
    enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin,
    systems::toggle_simulation,
};

pub mod player;
pub mod score;
pub mod star;
pub mod systems;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOverEvent>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(ScorePlugin)
            .add_system(handle_game_over)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}
pub struct GameOverEvent {
    pub score: u32,
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOverEvent>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your fnal score is: {}", event.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

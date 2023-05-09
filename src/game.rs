pub mod enemy;
use bevy::prelude::*;

use crate::AppState;

use self::{
    enemy::EnemyPlugin,
    player::PlayerPlugin,
    score::ScorePlugin,
    star::StarPlugin,
    systems::{pause_simulation, resume_simulation, toggle_simulation},
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
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(handle_game_over)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}
pub struct GameOverEvent {
    pub score: u32,
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.iter() {
        println!("Your fnal score is: {}", event.score);
        next_app_state.set(AppState::GameOver);
    }
}

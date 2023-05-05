use crate::AppState;
use bevy::prelude::*;

use self::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, despawn_enemies, enemy_movement, spawn_enemies,
        spawn_enemies_over_time, tick_enemy_spawn_timer, update_enemy_movement,
    },
};

use super::SimulationState;

pub mod component;
pub mod resources;
pub mod systems;

pub const ENEMY_SPEED: f32 = 300.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_movement,
                    confine_enemy_movement,
                    spawn_enemies_over_time,
                    tick_enemy_spawn_timer,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}

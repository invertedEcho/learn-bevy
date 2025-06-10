use bevy::prelude::*;
use resources::EnemySpawnTimer;
use systems::{
    confine_enemy_movement, despawn_enemies, enemy_movement, spawn_enemies,
    spawn_enemies_over_time, tick_enemy_spawn_timer, update_enemy_direction,
};

use crate::AppState;

use super::SimulationState;

pub mod components;
mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: u32 = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SPRITE_SIZE: f32 = 64.0;
pub const ENEMY_SPRITE_SRC: &str = "sprites/ball_red_large.png";
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

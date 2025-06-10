use bevy::prelude::*;
use systems::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player,
};

use crate::AppState;

use super::SimulationState;

mod components;
mod systems;

pub const PLAYER_SPRITE_SRC: &str = "sprites/ball_blue_large.png";
pub const PLAYER_SPRITE_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_movement.before(confine_player_movement),
                confine_player_movement.after(player_movement),
                enemy_hit_player,
                player_hit_star,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

use bevy::prelude::*;
use resources::StarSpawnTimer;
use systems::{despawn_stars, spawn_stars, spawn_stars_over_time, tick_star_spawn_timer};

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_STARS: u32 = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const STAR_SPRITE_SRC: &str = "sprites/star.png";

pub struct StarPlugin {}

pub mod components;
mod resources;
mod systems;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}

use bevy::prelude::*;
use resources::StarSpawnTimer;
use systems::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer};

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
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, tick_star_spawn_timer)
            .add_systems(Update, spawn_stars_over_time);
    }
}

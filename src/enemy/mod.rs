use bevy::prelude::*;
use resources::EnemySpawnTimer;
use systems::{
    confine_enemy_movement, enemy_movement, spawn_enemies, spawn_enemies_over_time,
    tick_enemy_spawn_timer, update_enemy_direction,
};

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
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, update_enemy_direction)
            .add_systems(Update, confine_enemy_movement)
            .add_systems(Update, tick_enemy_spawn_timer)
            .add_systems(Update, spawn_enemies_over_time);
    }
}

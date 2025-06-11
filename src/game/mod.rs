mod enemy;
mod player;
mod score;
mod star;
mod systems;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{AppState, GameOver};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins(PlayerPlugin {})
            .add_plugins(EnemyPlugin {})
            .add_plugins(StarPlugin {})
            .add_plugins(ScorePlugin {})
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(Update, transitition_to_game_state)
            .add_systems(Update, transition_to_main_menu_state)
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            ;
    }
}

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Default, Debug)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

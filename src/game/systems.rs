use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let current_simulation_state = *simulation_state.get();
        match current_simulation_state {
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
                println!("Set SimulationState to Paused.");
            }
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
                println!("Set SimulationState to Running.")
            }
        }
    }
}

pub fn transitition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            println!("Setting AppState to Game");
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            println!("Setting AppStateo to MainMenu");
            next_app_state.set(AppState::MainMenu);
        }
    }
}

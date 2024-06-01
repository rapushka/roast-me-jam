use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::constants::controls;

#[derive(Event)]
pub struct EndGame;

pub struct GameplayLoopPlugin;

impl Plugin for GameplayLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EndGame>()

            .add_systems(OnEnter(AppState::Gameplay), start_game)

            .add_systems(Update, end_game_with_esc)
            .add_systems(Update, end_game.run_if(on_event::<EndGame>()).run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Playing);
}

fn end_game_with_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<EndGame>,
) {
    if input.just_pressed(controls::PAUSE_KEY) {
        event.send(EndGame);
    }
}

fn end_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    next_app_state.set(AppState::MainMenu);
    next_game_state.set(GameState::Undefined);
}
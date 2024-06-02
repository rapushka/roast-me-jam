use bevy::prelude::*;
use crate::{AppState, constants, OnAppState, ui};
use crate::gameplay::gameplay_loop::game_over::GameOver;
use crate::gameplay::score::Score;
use crate::ui::EndGameButton;

pub fn build_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over_event: EventReader<GameOver>,
    score: Res<Score>,
) {
    for e in game_over_event.read() {
        let text = format!("You lost:(\ncuz: {}\nFinal score: {}", e.0, score.0);

        commands.spawn((
            Name::new("Game Over Screen"),
            NodeBundle {
                style: constants::styles::MAIN_MENU,
                ..default()
            },
            OnAppState(AppState::Gameplay),
        ))
            .with_children(|parent| {
                ui::create::light_text(&asset_server, text, parent, 64.0);
                ui::create::button(&asset_server, parent, "Back To Main Menu".to_string(), EndGameButton);
            });
        return;
    }
}
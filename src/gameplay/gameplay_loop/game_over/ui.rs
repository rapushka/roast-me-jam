use bevy::prelude::*;
use crate::{AppState, constants, OnAppState, ui};
use crate::gameplay::gameplay_loop::game_over::GameOver;
use crate::ui::EndGameButton;

pub fn build_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over_event: EventReader<GameOver>,
) {
    println!("len: {}", game_over_event.len());
    for e in game_over_event.read() {
        let text = format!("You lost:(\nbecause of: {}", e.0);

        commands.spawn((
            Name::new("Game Over Screen"),
            NodeBundle {
                style: constants::styles::MAIN_MENU,
                ..default()
            },
            OnAppState(AppState::Gameplay),
        ))
            .with_children(|parent| {
                ui::create::text(&asset_server, text, parent, 64.0);
                ui::create::button(&asset_server, parent, "Back To Main Menu".to_string(), EndGameButton);
            });
        return;
    }
}
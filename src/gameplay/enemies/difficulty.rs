use bevy::prelude::*;
use crate::{AppState, constants};

#[derive(Resource)]
pub struct Difficulty(pub f32);

pub struct DifficultyPlugin;

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Difficulty(1.0))

            .add_systems(OnEnter(AppState::Gameplay), reset_difficulty)
            .add_systems(Update, increase_difficulty.run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn reset_difficulty(
    mut difficulty: ResMut<Difficulty>,
) {
    difficulty.0 = 1.0;
}

fn increase_difficulty(
    mut difficulty: ResMut<Difficulty>,
    time: Res<Time>,
) {
    difficulty.0 += difficulty.0 * constants::DIFFICULTY_INCREMENT * time.delta_seconds();
}
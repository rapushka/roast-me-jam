use bevy::prelude::*;
use bevy_editor_pls::egui::debug_text::print;
use crate::{AppState, Order};
use crate::gameplay::enemies::components::{Enemy, EnemyType};
use crate::gameplay::enemies::difficulty::Difficulty;
use crate::gameplay::health::Kill;

#[derive(Resource, Default)]
pub struct Score(pub i32);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()

            .add_systems(OnEnter(AppState::Gameplay), reset_score)

            .add_systems(Update, (
                gain_score_from_kill_enemies
            )
                .run_if(in_state(AppState::Gameplay))
                .in_set(Order::Score))
        ;
    }
}

fn reset_score(
    mut score: ResMut<Score>,
) {
    score.0 = 0;
}

fn gain_score_from_kill_enemies(
    mut score: ResMut<Score>,
    difficulty: Res<Difficulty>,
    mut event: EventReader<Kill>,
    zombies: Query<&Enemy>,
) {
    for e in event.read() {
        if let Ok(enemy) = zombies.get(e.0) {
            let mut gain = match enemy.0 {
                EnemyType::Casual => 1,
            };

            gain *= (difficulty.0 * 100.0) as i32;
            score.0 += gain;

            println!("score: {}", score.0)
        }
    }
}
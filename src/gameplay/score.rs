use bevy::prelude::*;

use crate::{AppState, Order};
use crate::gameplay::enemies::components::{Enemy, EnemyType};
use crate::gameplay::enemies::difficulty::Difficulty;
use crate::gameplay::health::Kill;
use crate::gameplay::score::ui::{build_score_text, update_score_view};

pub mod ui;

#[derive(Resource, Default)]
pub struct Score(pub i32);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()

            .add_systems(OnEnter(AppState::Gameplay), (
                reset_score,
                build_score_text,
            ))

            .add_systems(Update, (
                gain_score_from_kill_enemies,
                update_score_view,
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
                EnemyType::Cone => 2,
                EnemyType::Bucked => 5,
            };

            gain *= (difficulty.0 * 100.0) as i32;
            score.0 += gain;
        }
    }
}
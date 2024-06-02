use bevy::prelude::*;
use components::EnemyType;
use crate::{AppState, constants};
use crate::gameplay::enemies::difficulty::DifficultyPlugin;
use crate::gameplay::enemies::spawn::*;

pub mod components;
pub mod spawn;
pub mod difficulty;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEnemy>()
            .add_event::<SpawnEnemyOnPosition>()

            .add_plugins(DifficultyPlugin)
            .insert_resource(SpawnEnemyTimer(Timer::from_seconds(0.0, TimerMode::Repeating)))

            .add_systems(OnEnter(AppState::Gameplay), reset_spawn_enemy_timer)

            // .add_systems(Update, test_spawn_enemy)

            .add_systems(Update, (
                spawn_default_enemy,
            ).run_if(on_event::<SpawnEnemy>()))

            .add_systems(Update, (
                spawn_enemy_on_position,
            ).run_if(on_event::<SpawnEnemyOnPosition>()))

            .add_systems(Update, (
                tick_spawn_enemy_timer,
            ).run_if(in_state(AppState::Gameplay)))
        ;
    }
}

fn test_spawn_enemy(
    input: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<SpawnEnemy>,
) {
    if input.just_pressed(KeyCode::Space) {
        event.send(SpawnEnemy(EnemyType::Casual));
    }
}
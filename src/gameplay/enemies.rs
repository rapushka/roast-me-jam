use bevy::prelude::*;
use crate::gameplay::enemies::spawn::{EnemyType, spawn_default_enemy, SpawnEnemy};

mod spawn;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEnemy>()

            .add_systems(Update, test_spawn_enemy)

            .add_systems(Update, (
                spawn_default_enemy,
            ).run_if(on_event::<SpawnEnemy>()))
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
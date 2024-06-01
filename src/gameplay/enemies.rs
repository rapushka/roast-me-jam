use bevy::prelude::*;
use crate::gameplay::enemies::spawn::{spawn_default_enemy, SpawnEnemy};

mod spawn;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEnemy>()

            .add_systems(Update, (
                spawn_default_enemy,
            ).run_if(on_event::<SpawnEnemy>()))
        ;
    }
}
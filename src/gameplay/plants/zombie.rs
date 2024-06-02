use bevy::prelude::*;

use crate::constants;
use crate::controls::Input;
use crate::gameplay::enemies::components::EnemyType;
use crate::gameplay::enemies::spawn::SpawnEnemyOnPosition;
use crate::gameplay::plants::{PlantType, SpawnPlant};

pub fn spawn(
    mut event: EventReader<SpawnPlant>,
    input: Res<Input>,
    mut event_spawn_enemy: EventWriter<SpawnEnemyOnPosition>,
) {
    for event in event.read() {
        if event.0 != PlantType::Zombie {
            continue;
        }

        if let Some(cursor_position) = input.mouse_world_position {
            let cursor_position = Vec3::new(cursor_position.x, cursor_position.y, constants::z_order::ENEMIES);

            event_spawn_enemy.send(SpawnEnemyOnPosition(EnemyType::Casual, cursor_position));
        }
    }
}
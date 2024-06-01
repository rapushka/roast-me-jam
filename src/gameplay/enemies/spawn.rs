use bevy::prelude::*;
use rand;
use rand::Rng;

use crate::{AppState, constants, OnAppState};
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::enemies::components::{Enemy, EnemyType};
use crate::gameplay::field::Field;
use crate::gameplay::movement::MovementSpeed;

#[derive(Event)]
pub struct SpawnEnemy(pub EnemyType);

pub fn spawn_default_enemy(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEnemy>,
    field: Res<Field>,
) {
    let mut rng = rand::thread_rng();

    for e in event_reader.read() {
        let enemy_type = e.0.clone();
        if enemy_type != EnemyType::Casual {
            continue;
        }

        let y_range = field.zombie_spawn_y_range.clone();
        let y = rng.gen_range(y_range);

        let start_position = Vec3::new(field.zombies_spawn_x, y, constants::z_order::ENEMIES);
        let speed = rng.gen_range(constants::CASUAL_ZOMBIE_MOVEMENT_SPEED);

        let entity = commands.spawn(Name::new("enemy"))
            .insert(Enemy(enemy_type))
            .insert(OnAppState(AppState::Gameplay))
            .insert(MovementSpeed(speed))
            .id();

        commands.add(AddAnimationCommand {
            entity,
            frames_count: 2,
            path_to_atlas: "sprites/default_enemy_atlas.png",
            fps: 4.0,
            layout: TextureAtlasLayout::from_grid(Vec2::new(125.0, 250.0), 2, 1, None, None),
            transform: Transform::from_translation(start_position).with_scale(Vec3::splat(0.5)),
        });
    }
}
use std::ops::Deref;
use std::time::Duration;
use bevy::prelude::*;
use rand;
use rand::Rng;

use crate::{AppState, constants, OnAppState};
use crate::constants::{BUCKET_ZOMBIE_HEALTH, CASUAL_ZOMBIE_HEALTH, CONE_ZOMBIE_HEALTH};
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::collisions::components::*;
use crate::gameplay::enemies::components::{Enemy, EnemyType};
use crate::gameplay::enemies::difficulty::Difficulty;
use crate::gameplay::field::Field;
use crate::gameplay::health::Health;
use crate::gameplay::movement::MovementSpeed;

#[derive(Resource)]
pub struct SpawnEnemyTimer(pub Timer);

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

        let health = match enemy_type {
            EnemyType::Casual => CASUAL_ZOMBIE_HEALTH,
            EnemyType::Cone => CONE_ZOMBIE_HEALTH,
            EnemyType::Bucked => BUCKET_ZOMBIE_HEALTH,
        };

        let y_range = field.zombie_spawn_y_range.clone();
        let y = rng.gen_range(y_range);

        let start_position = Vec3::new(field.zombies_spawn_x, y, constants::z_order::ENEMIES);
        let speed = rng.gen_range(constants::CASUAL_ZOMBIE_MOVEMENT_SPEED);

        let entity = commands.spawn(Name::new("enemy"))
            .insert(Enemy(enemy_type))
            .insert(OnAppState(AppState::Gameplay))
            .insert(MovementSpeed(speed))
            .insert(CircleCollider::new(100.0))
            .insert(Health(health))
            .id();

        let path = match enemy_type {
            EnemyType::Casual => "sprites/casual_enemy_atlas.png",
            EnemyType::Cone => "sprites/cone_enemy_atlas.png",
            EnemyType::Bucked => "sprites/bucket_enemy_atlas.png",
        };
        commands.add(AddAnimationCommand {
            entity,
            frames_count: 2,
            path_to_atlas: path,
            fps: 2.0,
            layout: TextureAtlasLayout::from_grid(Vec2::new(1024.0, 2048.0), 2, 1, None, None),
            transform: Transform::from_translation(start_position).with_scale(Vec3::splat(0.1)),
        });
    }
}

pub fn reset_spawn_enemy_timer(
    mut spawn_enemy_timer: ResMut<SpawnEnemyTimer>,
) {
    let duration = constants::SPAWN_FIRST_ENEMY_DURATION;
    spawn_enemy_timer.0.set_duration(Duration::from_secs_f32(duration));
}

pub fn tick_spawn_enemy_timer(
    mut spawn_enemy_timer: ResMut<SpawnEnemyTimer>,
    time: Res<Time>,
    mut event: EventWriter<SpawnEnemy>,
    difficulty: Res<Difficulty>,
) {
    spawn_enemy_timer.0.tick(time.delta());

    if spawn_enemy_timer.0.finished() {
        let mut rng = rand::thread_rng();
        let chance = rng.gen_range(0.0..=difficulty.0);

        let enemy_type = if chance > 5.0 {
            EnemyType::Bucked
        } else if chance > 1.0 {
            EnemyType::Cone
        } else {
            EnemyType::Casual
        };

        event.send(SpawnEnemy(enemy_type));

        let duration = rng.gen_range(constants::SPAWN_ENEMY_DURATION_RANGE);
        let duration = duration * (1.0 / difficulty.0);

        spawn_enemy_timer.0.set_duration(Duration::from_secs_f32(duration));
    }
}
use std::cmp::PartialEq;

use bevy::prelude::*;

use crate::{AppState, OnAppState};
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::field::Field;

#[derive(PartialEq, Clone, Copy)]
pub enum EnemyType {
    Casual,
}

#[derive(Component)]
pub struct Enemy(EnemyType);

#[derive(Event)]
pub struct SpawnEnemy(pub EnemyType);

pub fn spawn_default_enemy(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEnemy>,
    field: Res<Field>,
) {
    for e in event_reader.read() {
        let enemy_type = e.0.clone();
        if enemy_type != EnemyType::Casual {
            continue;
        }

        let start_position = Vec3::new(field.zombies_spawn_x, 0.0, 0.0);
        println!("{}", start_position);

        let entity = commands.spawn(Name::new("enemy"))
            .insert(Enemy(enemy_type))
            .insert(OnAppState(AppState::Gameplay))
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
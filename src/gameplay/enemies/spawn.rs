use std::cmp::PartialEq;

use bevy::prelude::*;

use crate::{AppState, OnAppState};
use crate::gameplay::animations::AddAnimationCommand;

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
) {
    for e in event_reader.read() {
        let enemy_type = e.0.clone();
        if enemy_type != EnemyType::Casual {
            continue;
        }

        let entity = commands.spawn(Name::new("enemy"))
            .insert(Enemy(enemy_type))
            .insert(OnAppState(AppState::Gameplay))
            .insert(Transform::from_scale(Vec3::splat(1.0)))
            .id();

        commands.add(AddAnimationCommand {
            entity,
            frames_count: 2,
            path_to_atlas: "sprites/default_enemy_atlas.png",
            fps: 4.0,
            layout: TextureAtlasLayout::from_grid(Vec2::new(125.0, 250.0), 2, 1, None, None),
        });
    }
}
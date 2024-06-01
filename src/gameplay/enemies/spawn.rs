use std::cmp::PartialEq;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::{AppState, OnAppState};

#[derive(PartialEq, Clone, Copy)]
pub enum EnemyType {
    Default,
}

#[derive(Component)]
pub struct Enemy(EnemyType);

#[derive(Event)]
pub struct SpawnEnemy(EnemyType);

pub fn spawn_default_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SpawnEnemy>,
) {
    for e in event_reader.read() {
        let enemy_type = e.0.clone();
        if enemy_type != EnemyType::Default {
            continue;
        }

        commands.spawn(Name::new("enemy"))
            .insert(Enemy(enemy_type))
            .insert(OnAppState(AppState::Gameplay))
            .insert(SpriteBundle {
                texture: asset_server.load("sprites/default_enemy.png"),
                ..default()
            })
        ;
    }
}
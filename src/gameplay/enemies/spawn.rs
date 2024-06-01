use std::cmp::PartialEq;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use crate::{AppState, OnAppState};
use crate::gameplay::animations::{AnimationIndices, AnimationTimer};

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
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SpawnEnemy>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    for e in event_reader.read() {
        let enemy_type = e.0.clone();
        if enemy_type != EnemyType::Casual {
            continue;
        }

        let texture = asset_server.load("sprites/default_enemy_atlas.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(125.0, 250.0), 2, 1, None, None);
        let texture_atlas = texture_atlases.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 1 };

        commands.spawn(Name::new("enemy"))
            .insert(Enemy(enemy_type))
            .insert(OnAppState(AppState::Gameplay))
            .insert(SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas,
                    index: animation_indices.first,
                },
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..default()
            })
            .insert(AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .insert(animation_indices)
        ;
    }
}
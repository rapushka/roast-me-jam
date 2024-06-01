use bevy::ecs::system::Command;
use bevy::prelude::*;
use crate::{AppState, OnAppState};

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub struct AddAnimationCommand {
    pub entity: Entity,
    pub path_to_atlas: &'static str,
    pub layout: TextureAtlasLayout,
    pub frames_count: usize,
    pub fps: f32,
}

impl Command for AddAnimationCommand {
    fn apply(self, world: &mut World) {
        let animation_indices = AnimationIndices { first: 0, last: self.frames_count - 1 };

        let layout = world.resource_mut::<Assets<TextureAtlasLayout>>().add(self.layout);
        let texture = world.resource::<AssetServer>().load(self.path_to_atlas);

        world.resource_scope(|world, assets: Mut<AssetServer>| {
            world.entity_mut(self.entity)
                .insert(SpriteSheetBundle {
                    texture,
                    atlas: TextureAtlas {
                        layout: layout,
                        index: animation_indices.first,
                    },
                    ..default()
                })
                .insert(AnimationTimer(Timer::from_seconds(1.0 / self.fps, TimerMode::Repeating)))
                .insert(animation_indices)
            ;
        });
    }
}

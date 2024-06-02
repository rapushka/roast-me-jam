use bevy::core::Name;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, EventReader, Res, TextureAtlasLayout, Transform};

use crate::{AppState, OnAppState};
use crate::controls::Input;
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::plants::{Plant, PlantType, SpawnPlant};
use crate::utils::Vec2Ext;

pub fn spawn(
    mut commands: Commands,
    mut event: EventReader<SpawnPlant>,
    input: Res<Input>,
) {
    for event in event.read() {
        if event.0 != PlantType::SkibidiToilet {
            continue;
        }

        if let Some(cursor_position) = input.mouse_world_position {
            let entity = commands.spawn(Name::new("skibidi toilet plant"))
                .insert(Plant(PlantType::SkibidiToilet))
                .insert(OnAppState(AppState::Gameplay))
                .insert(CircleCollider::new(75.0))
                .id()
                ;

            commands.add(AddAnimationCommand {
                entity,
                transform: Transform::from_translation(cursor_position.as_vec3()).with_scale(Vec3::splat(0.05)),
                path_to_atlas: "sprites/plants/skibidi toilet.png",
                layout: TextureAtlasLayout::from_grid(Vec2::new(2048.0, 2048.0), 1, 1, None, None),
                fps: 8.0,
                frames_count: 2,
            });
        }
    }
}
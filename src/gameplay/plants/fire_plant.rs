use bevy::prelude::*;

use crate::controls::Input;
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::plants::{Plant, PlantType, SpawnPlant};
use crate::{AppState, OnAppState};
use crate::gameplay::collisions::components::*;
use crate::utils::Vec2Ext;

pub fn spawn(
    mut commands: Commands,
    mut event: EventReader<SpawnPlant>,
    input: Res<Input>,
) {
    for event in event.read() {
        if event.0 != PlantType::Fire {
            continue;
        }

        if let Some(cursor_position) = input.mouse_world_position {
            let entity = commands.spawn(Name::new("fire plant"))
                .insert(Plant(PlantType::Fire))
                .insert(OnAppState(AppState::Gameplay))
                .insert(CircleCollider::new(75.0))
                .id()
                ;

            commands.add(AddAnimationCommand {
                entity,
                transform: Transform::from_translation(cursor_position.as_vec3()).with_scale(Vec3::splat(0.5)),
                path_to_atlas: "sprites/plants/fire_atlas.png",
                layout: TextureAtlasLayout::from_grid(Vec2::new(150.0, 200.0), 2, 1, None, None),
                fps: 8.0,
                frames_count: 2,
            });
        }
    }
}
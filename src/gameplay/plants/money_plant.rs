use bevy::core::Name;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, EventReader, Res, TextureAtlasLayout, Timer, TimerMode, Transform};
use crate::controls::Input;
use crate::gameplay::plants::{Plant, PlantType, SpawnPlant};
use crate::{AppState, OnAppState};
use crate::constants::{FIRE_DAMAGE, FIRE_TTL_IN_SECONDS};
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::health::CollisionDamage;
use crate::gameplay::plants::time_to_live::TimeToLive;
use crate::utils::Vec2Ext;

pub fn spawn(
    mut commands: Commands,
    mut event: EventReader<SpawnPlant>,
    input: Res<Input>,
) {
    for event in event.read() {
        if event.0 != PlantType::Money {
            continue;
        }

        if let Some(cursor_position) = input.mouse_world_position {
            let entity = commands.spawn(Name::new("money plant"))
                .insert(Plant(PlantType::Money))
                .insert(OnAppState(AppState::Gameplay))
                .insert(CircleCollider::new(75.0))
                .id()
                ;

            commands.add(AddAnimationCommand {
                entity,
                transform: Transform::from_translation(cursor_position.as_vec3()).with_scale(Vec3::splat(0.5)),
                path_to_atlas: "sprites/plants/money_flower_atlas.png",
                layout: TextureAtlasLayout::from_grid(Vec2::new(682.0, 882.0), 3, 1, None, None),
                fps: 4.0,
                frames_count: 2,
            });
        }
    }
}
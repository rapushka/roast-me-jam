use bevy::asset::AssetServer;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use crate::{AppState, constants, OnAppState};
use crate::controls::Clickable;
use crate::gameplay::field::Field;
use crate::gameplay::plants::PlantType;
use crate::gameplay::seeds::components::{Seed, SeedSlot};

pub fn spawn_seeds_slots(
    mut commands: Commands,
    field: Res<Field>,
) {
    let screen_center = field.seed_slots_position;

    let position = screen_center;
    commands.add(SpawnSeedsSlotCommand { position, seed: Some(PlantType::Fire) });
    let position = position + Vec2::new(100.0, 0.0);
    commands.add(SpawnSeedsSlotCommand { position, seed: None });
    let position = position + Vec2::new(100.0, 0.0);
    commands.add(SpawnSeedsSlotCommand { position, seed: None });
}

struct SpawnSeedsSlotCommand {
    pub position: Vec2,
    pub seed: Option<PlantType>,
}

impl Command for SpawnSeedsSlotCommand {
    fn apply(self, world: &mut World) {
        let texture = world.resource::<AssetServer>().load("sprites/seed_slot.png");

        let pos = Vec3::new(self.position.x, self.position.y, constants::z_order::SEED_SLOT);

        world.spawn(Name::new("seed slot"))
            .insert(SeedSlot(self.seed))
            .insert(OnAppState(AppState::Gameplay))
            .insert(SpriteBundle {
                texture,
                transform: Transform::from_translation(pos),
                ..default()
            })
            .insert(Clickable)
        ;
    }
}

pub fn fill_seed_slots(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    slots: Query<(Entity, &SeedSlot), Added<SeedSlot>>,
) {
    for (entity, seed) in slots.iter() {
        if let Some(plant_type) = seed.0 {
            commands.entity(entity)
                .with_children(|parent| {
                    parent
                        .spawn(Name::new("seed"))
                        .insert(Seed(plant_type))
                        .insert(SpriteBundle {
                            texture: asset_server.load("sprites/seed.png"),
                            ..default()
                        })
                    ;
                });
        }
    }
}
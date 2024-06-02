use bevy::asset::AssetServer;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use crate::{AppState, constants, OnAppState};
use crate::controls::Clickable;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::field::Field;
use crate::gameplay::plants::*;
use crate::gameplay::plants::price;
use crate::gameplay::plants::price::Price;
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

    assert_eq!(constants::SEED_SLOT_COUNT, 3);
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
                        .insert(Clickable)
                        .insert(Price(price::get_price(plant_type)))
                        .insert(SpriteBundle {
                            texture: asset_server.load("sprites/seed.png"),
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                            ..default()
                        })
                        .insert(CircleCollider::new(50.0))
                    ;
                });
        }
    }
}

pub fn seed_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    slots: Query<(Entity, &Seed), Added<Seed>>,
) {
    for (entity, seed) in slots.iter() {
        commands.entity(entity)
            .with_children(|parent| {
                parent
                    .spawn(Name::new("plant preview"))
                    .insert(PlantPreview(seed.0))
                    .insert(SpriteBundle {
                        texture: asset_server.load(get_sprite(seed.0)),
                        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 1.0)).with_scale(Vec3::splat(0.5)),
                        ..default()
                    })
                ;
            });
    }
}

pub fn get_sprite(plant_type: PlantType) -> &'static str {
    match plant_type {
        PlantType::Fire => "sprites/plants/fire_2.png"
    }
} 
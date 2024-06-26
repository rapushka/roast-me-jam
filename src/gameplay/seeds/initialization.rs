use bevy::asset::AssetServer;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use rand::Rng;
use crate::{AppState, constants, OnAppState};
use crate::controls::Clickable;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::field::Field;
use crate::gameplay::plants::*;
use crate::gameplay::plants::price;
use crate::gameplay::plants::price::Price;
use crate::gameplay::seeds::components::{Seed, SeedSlot};
use crate::gameplay::seeds::planting::SeedInHand;

pub fn spawn_seeds_slots(
    mut commands: Commands,
    field: Res<Field>,
) {
    let screen_center = field.seed_slots_position;

    let position = screen_center;
    commands.add(SpawnSeedsSlotCommand { position, seed: Some(PlantType::Fire) });
    let position = position + Vec2::new(100.0, 0.0);
    commands.add(SpawnSeedsSlotCommand { position, seed: Some(PlantType::Money) });
    let position = position + Vec2::new(100.0, 0.0);
    commands.add(SpawnSeedsSlotCommand { position, seed: Some(PlantType::Lego) });

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
    slots: Query<(Entity, &SeedSlot), Changed<SeedSlot>>,
) {
    for (entity, seed) in slots.iter() {
        if let Some(plant_type) = seed.0 {
            commands.entity(entity)
                .with_children(|parent| {
                    parent
                        .spawn(Name::new("seed"))
                        .insert(Seed(plant_type))
                        .insert(SeedInHand)
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
                        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 1.0)).with_scale(get_scale(seed.0)),
                        ..default()
                    })
                ;
            });
    }
}

pub fn get_scale(plant_type: PlantType) -> Vec3 {
    match plant_type {
        PlantType::Fire => Vec3::splat(0.04),
        PlantType::Money => Vec3::splat(0.1),
        PlantType::Lego => Vec3::splat(0.04),
        PlantType::SkibidiToilet => Vec3::splat(0.04),
        PlantType::Weezer => Vec3::splat(0.2),
        PlantType::SkibidiToiletLefthand => Vec3::splat(0.04),
        PlantType::Zombie => Vec3::splat(0.04),
    }
}

pub fn get_sprite(plant_type: PlantType) -> &'static str {
    match plant_type {
        PlantType::Fire => "sprites/plants/fire_1.png",
        PlantType::Money => "sprites/plants/money_flower.png",
        PlantType::Lego => "sprites/plants/lego.png",
        PlantType::SkibidiToilet => "sprites/plants/skibidi toilet.png",
        PlantType::Weezer => "sprites/plants/weezer.png",
        PlantType::SkibidiToiletLefthand => "sprites/plants/skibidi toilet left.png",
        PlantType::Zombie => "sprites/casual_enemy.png",
    }
}

pub fn pick_random_plant() -> PlantType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=6) {
        0 => PlantType::Fire,
        1 => PlantType::Money,
        2 => PlantType::Lego,
        3 => PlantType::SkibidiToilet,
        4 => PlantType::Weezer,
        5 => PlantType::SkibidiToiletLefthand,
        6 => PlantType::Zombie,
        _ => panic!("nah:("),
    }
} 


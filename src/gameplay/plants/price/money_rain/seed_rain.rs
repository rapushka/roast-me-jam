use bevy::prelude::*;
use rand::Rng;

use crate::{AppState, constants, OnAppState};
use crate::controls::Clickable;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::movement::move_to_target::MoveToTarget;
use crate::gameplay::movement::MovementSpeed;
use crate::gameplay::plants::PlantType;
use crate::gameplay::plants::time_to_live::TimeToLive;
use crate::gameplay::seeds::components::Seed;

#[derive(Event)]
pub struct DropSeed {
    pub start_position: Vec3,
    pub end_position: Vec3,
}

pub struct SeedRainPlugin;

impl Plugin for SeedRainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DropSeed>()

            .add_systems(Update, (
                drop_seed,
            ).run_if(on_event::<DropSeed>()))
        ;
    }
}

fn drop_seed(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event: EventReader<DropSeed>,
) {
    for e in event.read() {
        let plant_type = pick_random_plant();

        commands.spawn(Name::new("seed"))
            .insert(Seed(plant_type))
            .insert(SpriteBundle {
                texture: asset_server.load("sprites/seed.png"),
                ..default()
            })
            .insert(Transform::from_translation(e.start_position).with_scale(Vec3::splat(0.5)))
            .insert(MoveToTarget(e.end_position))
            .insert(MovementSpeed(constants::MONEY_FALL_SPEED))
            .insert(TimeToLive(Timer::from_seconds(constants::MONEY_TTL, TimerMode::Once)))
            .insert(OnAppState(AppState::Gameplay))
            .insert(Clickable)
            .insert(CircleCollider::new(30.0))
        ;
    }
}

fn pick_random_plant() -> PlantType {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => PlantType::Fire,
        1 => PlantType::Money,
        2 => PlantType::Lego,
        _ => panic!("nah:("),
    }
}
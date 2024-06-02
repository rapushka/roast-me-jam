use bevy::core::Name;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy_editor_pls::egui::debug_text::print;
use rand::Rng;

use crate::{AppState, constants, OnAppState};
use crate::controls::Input;
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::collisions::components::CircleCollider;
use crate::gameplay::plants::{Plant, PlantType, SpawnPlant};
use crate::gameplay::plants::price::money_rain::SpawnMoney;
use crate::utils::Vec2Ext;

#[derive(Component)]
pub struct MoneyPlantDropMoney(pub Timer);

#[derive(Event)]
pub struct SpawnMoneyFromTree(pub Vec3);

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
                .insert(MoneyPlantDropMoney(Timer::from_seconds(constants::MONEY_PLANT_HARVEST, TimerMode::Repeating)))
                .id()
                ;

            commands.add(AddAnimationCommand {
                entity,
                transform: Transform::from_translation(cursor_position.as_vec3()).with_scale(Vec3::splat(0.1)),
                path_to_atlas: "sprites/plants/money_flower_atlas.png",
                layout: TextureAtlasLayout::from_grid(Vec2::new(682.0, 882.0), 3, 1, None, None),
                fps: 4.0,
                frames_count: 2,
            });
        }
    }
}

pub fn tick_money_plant_harvest(
    mut plants: Query<(&mut MoneyPlantDropMoney, &Transform)>,
    time: Res<Time>,
    mut event_writer: EventWriter<SpawnMoneyFromTree>,
) {
    for (mut harvest_timer, transform) in plants.iter_mut() {
        let mut harvest_timer = &mut harvest_timer.0;
        harvest_timer.tick(time.delta());

        if harvest_timer.finished() {
            event_writer.send(SpawnMoneyFromTree(transform.translation));
        }
    }
}

pub fn spawn_money_from_tree(
    mut event: EventReader<SpawnMoneyFromTree>,
    mut spawn_money_event: EventWriter<SpawnMoney>,
) {
    for e in event.read() {
        let mut start_position = e.0;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(constants::MONEY_PLANT_SPAWN_MONEY_X);
        let y = rng.gen_range(constants::MONEY_PLANT_SPAWN_MONEY_Y);
        start_position.x += x;
        start_position.y += y;
        let mut end_position = start_position;
        end_position.y -= constants::MONEY_PLANT_SPAWN_MONEY_FALL_DISTANCE;

        spawn_money_event.send(SpawnMoney {
            start_position,
            end_position,
        });
    }
}
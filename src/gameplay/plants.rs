use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::plants::money_plant::{SpawnMoneyFromTree, tick_money_plant_harvest};
use crate::gameplay::plants::price::PricesPlugin;
use crate::gameplay::plants::time_to_live::update_time_to_live;

mod time_to_live;
pub mod price;
mod fire_plant;
mod money_plant;

#[derive(Event)]
pub struct SpawnPlant(pub PlantType);

#[derive(Component)]
pub struct Plant(pub PlantType);

#[derive(Component)]
pub struct PlantPreview(pub PlantType);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlantType {
    Fire,
    Money,
}

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPlant>()
            .add_event::<SpawnMoneyFromTree>()

            .add_plugins((
                PricesPlugin,
            ))

            // .add_systems(Update, test_spawn)

            .add_systems(Update, (
                update_time_to_live,
                tick_money_plant_harvest,
            ).run_if(in_state(AppState::Gameplay)))

            .add_systems(Update, (
                fire_plant::spawn,
                money_plant::spawn,
            )
                .run_if(on_event::<SpawnPlant>()))
        ;
    }
}

pub fn test_spawn(
    buttons: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<SpawnPlant>,
) {
    if buttons.just_pressed(KeyCode::KeyF) {
        event.send(SpawnPlant(PlantType::Fire));
    }
}
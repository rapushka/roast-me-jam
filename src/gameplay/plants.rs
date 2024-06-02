use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::plants::money_plant::{spawn_money_from_tree, SpawnMoneyFromTree, tick_money_plant_harvest};
use crate::gameplay::plants::price::PricesPlugin;
use crate::gameplay::plants::projectiles::{move_projectiles_to_right, shoot, Shoot, shoot_periodically};
use crate::gameplay::plants::time_to_live::update_time_to_live;

pub mod plant_preview;
pub mod time_to_live;
pub mod price;
pub mod projectiles;

mod fire_plant;
mod money_plant;
mod lego_plant;
mod skibidi_toilet_plant;
mod weezer;

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
    Lego,
    SkibidiToilet,
    Weezer,
}

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPlant>()
            .add_event::<SpawnMoneyFromTree>()
            .add_event::<Shoot>()

            .add_plugins((
                PricesPlugin,
            ))

            // .add_systems(Update, test_spawn)

            .add_systems(Update, (
                update_time_to_live,

                // money plant
                tick_money_plant_harvest,
                spawn_money_from_tree,

                // projectiles
                move_projectiles_to_right,
                shoot_periodically,
                shoot,
            ).run_if(in_state(AppState::Gameplay)))

            .add_systems(Update, (
                fire_plant::spawn,
                money_plant::spawn,
                lego_plant::spawn,
                skibidi_toilet_plant::spawn,
                weezer::spawn,
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
use bevy::prelude::*;
use crate::{AppState, constants};
use crate::gameplay::plants::PlantType;
use crate::gameplay::plants::price::current_money::*;
use crate::gameplay::plants::price::money_rain::MoneyRainPlugin;
use crate::gameplay::plants::price::spend::SpendPlugin;
use crate::gameplay::seeds::components::Seed;

mod spawn;
pub mod current_money;
pub mod spend;
pub mod money_rain;

#[derive(Component)]
pub struct Price(pub i32);

pub struct PricesPlugin;

impl Plugin for PricesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                SpendPlugin,
                MoneyRainPlugin,
            ))
            .add_systems(OnEnter(AppState::Gameplay), spawn_current_money)

            .add_systems(Update, (
                visualise_plant_price,
                update_current_money,
            ))
        ;
    }
}

fn visualise_plant_price(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seeds: Query<(Entity, &Price), Added<Seed>>,
) {
    for (e, price) in seeds.iter() {
        commands.entity(e)
            .with_children(|parent| {
                let transform = Transform::from_xyz(0.0, -40.0, 2.0);
                spawn::price_text(&asset_server, parent, price.0, transform);
            });
    }
}

pub fn get_price(plant_type: PlantType) -> i32 {
    match plant_type {
        PlantType::Fire => constants::FIRE_PRICE,
        PlantType::Money => constants::MONEY_PLANT_PRICE,
        PlantType::Lego => constants::LEGO_PRICE,
        PlantType::SkibidiToilet => constants::SKIBIDI_TOILET_PRICE,
        PlantType::Weezer => constants::WEEZER_PRICE,
        PlantType::SkibidiToiletLefthand => constants::SKIBIDI_TOILET_LEFT_PRICE,
        PlantType::Zombie => constants::ZOMBIE_PRICE,
    }
}

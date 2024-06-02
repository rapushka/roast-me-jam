use bevy::prelude::*;

use crate::gameplay::plants::price::current_money::CurrentMoney;
use crate::gameplay::plants::price::get_price;
use crate::gameplay::plants::SpawnPlant;
use crate::gameplay::seeds::planting::{PlantingState, SelectedSeed};

pub struct SpendPlugin;

impl Plugin for SpendPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlantingState::Buying),
                         try_buy_plant,
            )

            .add_systems(Update, spend_money.run_if(on_event::<SpawnPlant>()))
        
        ;
    }
}

fn try_buy_plant(
    mut next_state: ResMut<NextState<PlantingState>>,
    selected_seed: Res<SelectedSeed>,
    current_money: Query<&CurrentMoney>,
) {
    let price = get_price(selected_seed.0.unwrap());

    let current_money = current_money.single();
    if current_money.0 >= price {
        next_state.set(PlantingState::Planting);
    } else {
        println!("nu-uh, no money");
        next_state.set(PlantingState::Harvesting);
    }
}

fn spend_money(
    mut plant_event: EventReader<SpawnPlant>,
    mut current_money: Query<&mut CurrentMoney>,
) {
    let mut current_money = current_money.single_mut();
    for plant in plant_event.read() {
        let price = get_price(plant.0);

        current_money.0 -= price;
    }
}
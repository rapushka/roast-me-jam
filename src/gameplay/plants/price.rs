use bevy::prelude::*;
use crate::AppState;
use crate::gameplay::plants::price::current_money::*;
use crate::gameplay::seeds::components::Seed;

mod spawn;
pub mod current_money;

#[derive(Component)]
pub struct Price(pub i32);

pub struct PricesPlugin;

impl Plugin for PricesPlugin {
    fn build(&self, app: &mut App) {
        app
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
                let transform = Transform::from_xyz(0.0, -46.0, 2.0);
                spawn::price_text(&asset_server, parent, price.0, transform);
            });
    }
}
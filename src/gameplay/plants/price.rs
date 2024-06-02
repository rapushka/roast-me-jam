use bevy::prelude::*;
use crate::gameplay::seeds::components::Seed;

mod spawn;

#[derive(Component)]
pub struct Price(pub i32);

pub struct PricesPlugin;

impl Plugin for PricesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, visualise_plant_price)
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

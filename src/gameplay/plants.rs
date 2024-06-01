use bevy::prelude::*;

mod fire_plant;

#[derive(Event)]
pub struct SpawnPlant(PlantType);

pub enum PlantType {
    Fire,
}

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPlant>()

            .add_systems(Update, (
                fire_plant::spawn,
            )
                .run_if(on_event::<SpawnPlant>()))
        ;
    }
}
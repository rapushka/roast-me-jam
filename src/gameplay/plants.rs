use bevy::prelude::*;
use crate::gameplay::plants::time_to_live::update_time_to_live;

mod time_to_live;
mod fire_plant;

#[derive(Event)]
pub struct SpawnPlant(pub PlantType);

#[derive(Component)]
pub struct Plant(pub PlantType);

#[derive(PartialEq, Clone, Copy)]
pub enum PlantType {
    Fire,
}

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPlant>()

            // .add_systems(Update, test_spawn)

            .add_systems(Update, update_time_to_live)

            .add_systems(Update, (
                fire_plant::spawn,
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
use bevy::prelude::*;

mod fire_plant;

#[derive(Event)]
pub struct SpawnPlant(PlantType);

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

            .add_systems(Update, test_spawn)

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
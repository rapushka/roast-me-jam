use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::AppState;
use crate::controls::Input;
use crate::gameplay::plants::{PlantType, SpawnPlant};
use crate::gameplay::seeds::components::Seed;
use crate::ui::Clicked;

#[derive(Resource, Default)]
pub struct SelectedSeed(pub Option<PlantType>);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PlantingState {
    #[default]
    Harvesting,
    Planting,
}

pub struct PlantingPlugin;

impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PlantingState>()

            .init_resource::<SelectedSeed>()

            .add_systems(Update, (
                start_planting,
            ).run_if(in_state(AppState::Gameplay)))

            .add_systems(Update, (
                plant,
            ).run_if(in_state(PlantingState::Planting)))

            .add_systems(OnExit(PlantingState::Planting), reset_selected_seed)
        ;
    }
}

fn start_planting(
    mut clicked_event: EventReader<Clicked>,
    seeds: Query<&Seed>,
    mut selected_seed: ResMut<SelectedSeed>,
    mut next_state: ResMut<NextState<PlantingState>>,
) {
    for e in clicked_event.read() {
        if let Ok(clicked_seed) = seeds.get(e.0) {
            if let Some(previous_plant_type) = selected_seed.0 {
                if previous_plant_type == clicked_seed.0 {
                    next_state.set(PlantingState::Harvesting);
                }
            }

            selected_seed.0 = Some(clicked_seed.0);
            next_state.set(PlantingState::Planting);
        }
    }
}

fn reset_selected_seed(
    mut selected_seed: ResMut<SelectedSeed>,
) {
    selected_seed.0 = None;
}

fn plant(
    input: Res<Input>,
    mut selected_seed: ResMut<SelectedSeed>,
    mut next_state: ResMut<NextState<PlantingState>>,
    mut spawn_plant_event: EventWriter<SpawnPlant>,
) {
    if input.left_click {
        let plant_type = selected_seed.0.unwrap();
        spawn_plant_event.send(SpawnPlant(plant_type));
        next_state.set(PlantingState::Harvesting);
    }
}
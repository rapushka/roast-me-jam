use bevy::app::{App, Plugin};
use bevy::audio::Volume;
use bevy::prelude::*;
use crate::AppState;
use crate::controls::Input;
use crate::gameplay::plants::{PlantType, SpawnPlant};
use crate::gameplay::plants::plant_preview::PlantPreview;
use crate::gameplay::seeds::components::Seed;
use crate::gameplay::seeds::planting::cooldowns::Cooldown;
use crate::ui::Clicked;

pub mod cooldowns;

#[derive(Resource, Default)]
pub struct SelectedSeed(pub Option<PlantType>);

#[derive(Component)]
pub struct SeedInHand;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PlantingState {
    #[default]
    Harvesting,
    Buying,
    Planting,
    Picking,
}

pub struct PlantingPlugin;

impl Plugin for PlantingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PlantingState>()

            .init_resource::<SelectedSeed>()

            .add_systems(OnEnter(AppState::Gameplay), init_planting_state)

            .add_systems(Update, (
                start_planting,
            ).run_if(in_state(AppState::Gameplay))
                .run_if(not(in_state(PlantingState::Picking))))

            .add_systems(Update, (
                plant,
            ).run_if(in_state(PlantingState::Planting)))

            .add_systems(Update, cancel_with_rmb.run_if(not(in_state(PlantingState::Harvesting))))

            .add_systems(OnEnter(PlantingState::Harvesting), reset_selected_seed)
        ;
    }
}

fn init_planting_state(
    mut next_state: ResMut<NextState<PlantingState>>,
) {
    next_state.set(PlantingState::Harvesting);
}

fn start_planting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut clicked_event: EventReader<Clicked>,
    seeds: Query<&Seed, (With<SeedInHand>, Without<Cooldown>)>,
    mut selected_seed: ResMut<SelectedSeed>,
    mut next_state: ResMut<NextState<PlantingState>>,
) {
    for e in clicked_event.read() {
        if let Ok(clicked_seed) = seeds.get(e.0) {
            if let Some(previous_plant_type) = selected_seed.0 {
                if previous_plant_type == clicked_seed.0 {
                    next_state.set(PlantingState::Harvesting);

                    commands.spawn(
                        AudioBundle {
                            source: asset_server.load("audio/pop.ogg"),
                            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.1)),
                        }
                    );

                    continue;
                }
            }

            selected_seed.0 = Some(clicked_seed.0);
            next_state.set(PlantingState::Buying);
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
    previews: Query<&PlantPreview>,
) {
    if input.left_click {
        let preview = previews.get_single().unwrap();

        if preview.can_plant {
            let plant_type = selected_seed.0.unwrap();
            spawn_plant_event.send(SpawnPlant(plant_type));
        } else {}
        next_state.set(PlantingState::Harvesting);
    }
}

fn cancel_with_rmb(
    input: Res<Input>,
    mut next_planting_state: ResMut<NextState<PlantingState>>,
) {
    if input.right_click {
        next_planting_state.set(PlantingState::Harvesting);
    }
}
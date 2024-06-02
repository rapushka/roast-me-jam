use bevy::prelude::*;
use crate::gameplay::plants::price::money_rain::seed_rain::DroppedSeed;
use crate::gameplay::seeds::components::{Seed, SeedSlot};
use crate::gameplay::seeds::planting::{PlantingState, SeedInHand, SelectedSeed};
use crate::ui::Clicked;

#[derive(Component)]
pub struct PickingDroppedSeed;

pub struct PickingDroppedSeedPlugin;

impl Plugin for PickingDroppedSeedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                click_on_dropped_seed,
            ).run_if(on_event::<Clicked>()))

            .add_systems(Update, flicker_seeds_in_hand.run_if(in_state(PlantingState::Picking)))
            .add_systems(OnExit(PlantingState::Picking), reset_seeds_color)

            .add_systems(Update, replace_seed.run_if(in_state(PlantingState::Picking)))
        ;
    }
}

fn click_on_dropped_seed(
    mut commands: Commands,
    mut event_reader: EventReader<Clicked>,
    dropped_seed: Query<Entity, With<DroppedSeed>>,
    mut next_picking_state: ResMut<NextState<PlantingState>>,
) {
    for e in event_reader.read() {
        if let Ok(seed) = dropped_seed.get(e.0) {
            next_picking_state.set(PlantingState::Picking);
            commands.entity(seed).insert(PickingDroppedSeed);
        }
    }
}

fn flicker_seeds_in_hand(
    mut seeds: Query<&mut Sprite, With<SeedInHand>>,
    time: Res<Time>,
) {
    for mut sprite in seeds.iter_mut() {
        let time = time.elapsed_seconds();
        let frequency = 10.0;
        let grayscale = ((frequency * time).sin() + 1.0) / 2.0;
        let grayscale = grayscale + 0.5;

        sprite.color = Color::rgb(grayscale, grayscale, grayscale);
    }
}

fn reset_seeds_color(
    mut seeds: Query<&mut Sprite, With<SeedInHand>>,
) {
    for mut sprite in seeds.iter_mut() {
        sprite.color = Color::WHITE;
    }
}

fn replace_seed(
    mut commands: Commands,
    mut clicked_event: EventReader<Clicked>,
    mut seeds: Query<(Entity, &Parent), With<SeedInHand>>,
    picked_seeds: Query<(Entity, &Seed), With<PickingDroppedSeed>>,
    mut slots: Query<&mut SeedSlot>,
    mut next_state: ResMut<NextState<PlantingState>>,
) {
    for e in clicked_event.read() {
        if let Ok((clicked_seed_entity, slot)) = seeds.get_mut(e.0) {
            for (picked_entity, picked) in picked_seeds.iter() {
                let mut slot = slots.get_mut(slot.get()).unwrap();
                slot.0 = Some(picked.0);

                commands.entity(clicked_seed_entity).despawn_recursive();
                commands.entity(picked_entity).despawn_recursive();
            }

            next_state.set(PlantingState::Harvesting);
        }
    }
}
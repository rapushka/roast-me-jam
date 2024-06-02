use bevy::prelude::*;
use crate::gameplay::plants::price::money_rain::seed_rain::DroppedSeed;
use crate::gameplay::seeds::components::Seed;
use crate::gameplay::seeds::planting::{PlantingState, SeedInHand};
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
        ;
    }
}

fn click_on_dropped_seed(
    mut event_reader: EventReader<Clicked>,
    dropped_seed: Query<&Seed, With<DroppedSeed>>,
    mut next_picking_state: ResMut<NextState<PlantingState>>,
) {
    for e in event_reader.read() {
        if let Ok(seed) = dropped_seed.get(e.0) {
            next_picking_state.set(PlantingState::Picking);
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
use bevy::prelude::*;
use crate::gameplay::plants::PlantType;

#[derive(Component)]
pub struct SeedSlot(pub Option<PlantType>);
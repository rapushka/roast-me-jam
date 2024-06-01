use bevy::prelude::*;

#[derive(Component)]
pub struct SeedSlot(pub Option<Entity>);
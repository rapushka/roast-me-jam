use bevy::prelude::*;

#[derive(Component)]
pub struct Health(f32);

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
        ;
    }
}
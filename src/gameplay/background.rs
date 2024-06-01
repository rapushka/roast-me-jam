use bevy::prelude::*;

use crate::{AppState, OnAppState};
use crate::gameplay::field::Field;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameplay), (
                spawn_background,
            ))
        ;
    }
}

fn spawn_background(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    field: Res<Field>,
) {
    commands.spawn(Name::new("background"))
        .insert(OnAppState(AppState::Gameplay))
        .insert(SpriteBundle {
            texture: asset_server.load("sprites/background.png"),
            ..default()
        })
        .insert(Transform::from_translation(field.screen_center.into()))
    ;
}
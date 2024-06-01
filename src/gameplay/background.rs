use bevy::prelude::*;

use crate::{AppState, OnAppState};

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
) {
    commands.spawn(Name::new("background"))
        .insert(OnAppState(AppState::Gameplay))
        .insert(SpriteBundle {
            texture: asset_server.load("sprites/background.png"),
            ..default()
        })
    ;
}
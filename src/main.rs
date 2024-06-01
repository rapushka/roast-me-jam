use crate::controls::ControlsPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_editor_pls::EditorPlugin;
use bevy_text_animation::TextAnimatorPlugin;
use crate::gameplay::GameplayPlugin;
use crate::ui::UiPlugin;

mod constants;
mod ui;
mod gameplay;
pub mod controls;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Order {
    Input,
    GameLogic,
    View,
    Cleanups,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Gameplay,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Undefined,
    Playing,
    Paused,
    GameOver,
}

#[derive(Component)]
pub struct OnAppState(pub AppState);

fn main() {
    App::new()
        .configure_sets(Update, (Order::Input, Order::GameLogic, Order::View, Order::Cleanups).chain())

        .init_state::<AppState>()
        .init_state::<GameState>()

        .add_plugins((
            // Dependencies
            DefaultPlugins,
            EditorPlugin::default(),
            TextAnimatorPlugin,
        ))

        .add_plugins((
            // Game
            UiPlugin,
            GameplayPlugin,
            ControlsPlugin,
        ))

        .add_systems(OnEnter(AppState::Loading), (
            spawn_camera,
            start_game,
        ))

        .add_systems(Update, (
            despawn_not_in_state,
        ))

        .run();
}

fn start_game(
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::MainMenu);
}

fn spawn_camera(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();

    commands.spawn(Name::new("camera"))
        .insert(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        })
        .insert(IsDefaultUiCamera)
    ;
}

pub fn despawn_not_in_state(
    mut transitions: EventReader<StateTransitionEvent<AppState>>,
    mut entities: Query<(Entity, &OnAppState)>,
    mut commands: Commands,
) {
    for transition in transitions.read() {
        for (entity, on_state) in &mut entities {
            if on_state.0 != transition.after {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

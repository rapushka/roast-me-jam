use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_text_animation::TextAnimatorPlugin;
use crate::gameplay::GameplayPlugin;
use crate::ui::UiPlugin;

mod constants;
mod ui;
mod gameplay;

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
        ))

        .add_systems(OnEnter(AppState::Loading), (
            spawn_camera,
            start_game,
        ))

        .run();
}

fn start_game(
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::MainMenu);
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Name::new("camera"))
        .insert(Camera2dBundle::default())
        .insert(IsDefaultUiCamera)
    ;
}


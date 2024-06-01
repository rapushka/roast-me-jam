use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_text_animation::TextAnimatorPlugin;

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

        .add_systems(OnEnter(AppState::Loading), (
            start_game
        ))

        .run();
}

fn start_game() {}

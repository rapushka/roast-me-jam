use bevy::app::AppExit;
use bevy::prelude::*;
use crate::{AppState, constants, OnAppState, ui};
use crate::gameplay::field::Field;
use crate::ui::{Clicked, create};

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), (
                build_main_menu,
                spawn_background,
            ))

            .add_systems(Update, (
                on_quit_button_clicked,
                on_play_button_clicked,
            ).run_if(in_state(AppState::MainMenu)))

            .add_systems(OnExit(AppState::MainMenu), (
                destroy_main_menu,
            ))
        ;
    }
}

pub fn build_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("main menu page"),
        MainMenu {},
        NodeBundle {
            style: constants::styles::MAIN_MENU,
            z_index: ui::order::MAIN_MENU,
            ..default()
        },
    ))
        .with_children(|parent| {
            create::title(&asset_server, parent, "BrainRoasts vs PinosPrime".to_string());
            create::horizontal_layout(parent, |parent| {
                create::image_button(&asset_server, parent, PlayButton, "ui/play-button-arrowhead.png", 100.0);
                create::image_button(&asset_server, parent, QuitButton, "ui/exit.png", 100.0);
            }) ;
        });
}

pub fn destroy_main_menu(
    main_menus: Query<Entity, With<MainMenu>>,
    mut commands: Commands,
) {
    for main_menu in main_menus.iter() {
        commands.entity(main_menu).despawn_recursive();
    }
}

pub fn on_play_button_clicked(
    buttons: Query<Entity, With<PlayButton>>,
    mut event_reader: EventReader<Clicked>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.0) {
            next_state.set(AppState::Gameplay);
        }
    }
}

pub fn on_quit_button_clicked(
    buttons: Query<Entity, With<QuitButton>>,
    mut event_reader: EventReader<Clicked>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for event in event_reader.read() {
        if let Ok(_) = buttons.get(event.0) {
            app_exit_event.send(AppExit);
        }
    }
}

fn spawn_background(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    field: Res<Field>,
) {
    let screen_center = field.screen_center;
    let position = Vec3 {
        x: screen_center.x,
        y: screen_center.y,
        z: constants::z_order::BACKGROUND,
    };
    commands.spawn(Name::new("background"))
        .insert(OnAppState(AppState::MainMenu))
        .insert(SpriteBundle {
            texture: asset_server.load("sprites/background_main_menu.png"),
            ..default()
        })
        .insert(Transform::from_translation(position).with_scale(Vec3::splat(1.0)))
    ;
}
use bevy::prelude::*;
pub use main_menu::*;
use crate::{constants, gameplay};
use crate::gameplay::gameplay_loop::EndGame;

mod main_menu;
// mod gameplay_hud;
pub mod create;

pub mod order {
    use bevy::prelude::ZIndex;

    pub const LOADING_CURTAIN: ZIndex = ZIndex::Global(1_000);
    pub const PAUSE_MENU: ZIndex = ZIndex::Global(200);
    pub const MAIN_MENU: ZIndex = ZIndex::Global(100);
    pub const GAMEPLAY_HUD: ZIndex = ZIndex::Global(10);
}

#[derive(Event)]
pub struct Clicked(pub Entity);

#[derive(Component)]
pub struct EndGameButton;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Clicked>()

            .add_plugins((
                MainMenuPlugin,
                // LevelSelectionPlugin,
                // GameplayHudPlugin,
                // GameOverPlugin,
            ))

            .add_systems(Update, (
                visualise_interaction_with_buttons,
                click_on_pressed_button,
                click_on_end_game_button,
                // gameplay_hud::pause::on_back_button_clicked,
            ))
        ;
    }
}

pub fn visualise_interaction_with_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut background_color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = constants::color::PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *background_color = constants::color::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *background_color = constants::color::DEFAULT_BUTTON.into();
            }
        };
    }
}

pub fn click_on_pressed_button(
    mut buttons: Query<(&Interaction, Entity), Changed<Interaction>>,
    mut clicked_event: EventWriter<Clicked>,
) {
    for (interaction, entity) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                clicked_event.send(Clicked(entity));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        };
    }
}

pub fn click_on_end_game_button(
    mut clicked_event: EventReader<Clicked>,
    buttons: Query<Entity, (With<Button>, With<EndGameButton>)>,
    mut end_game_event: EventWriter<EndGame>,
) {
    for event in clicked_event.read() {
        if let Ok(_) = buttons.get(event.0) {
            end_game_event.send(EndGame);
        }
    }
} 
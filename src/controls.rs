use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::window::PrimaryWindow;

use crate::ui::Clicked;

#[derive(Resource, Default)]
pub struct Input {
    pub mouse_world_position: Option<Vec2>,
    pub left_click: bool,
}

#[derive(Component)]
pub struct Clickable;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Input>()

            .add_systems(Update, (
                mouse_position_to_world,
                track_mouse_clicks,
            ).chain())
        ;
    }
}

fn mouse_position_to_world(
    mut input: ResMut<Input>,
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let window = windows.single();

    input.mouse_world_position = match window.cursor_position() {
        Some(cursor_position) => Some(Vec2::new(cursor_position.x, window.height() - cursor_position.y)),
        None => None,
    };

    input.left_click = buttons.just_pressed(MouseButton::Left);
}

fn track_mouse_clicks(
    input: Res<Input>,
    mut clicked_event: EventWriter<Clicked>,
    clickable_entities: Query<(Entity, &Aabb, &Transform), With<Clickable>>,
) {
    if !input.left_click {
        return;
    }

    if let Some(cursor_position) = input.mouse_world_position {
        for (entity, aabb, transform) in clickable_entities.iter() {
            let min: Vec3 = aabb.min().into();
            let max: Vec3 = aabb.max().into();

            let min = min + transform.translation;
            let max = max + transform.translation;

            let intersects = cursor_position.x >= min.x && cursor_position.y >= min.y
                && cursor_position.x <= max.x && cursor_position.y <= max.y;

            if intersects {
                clicked_event.send(Clicked(entity));
            }
        }
    }
}
use bevy::input::ButtonInput;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::window::PrimaryWindow;
use crate::ui::Clicked;

#[derive(Component)]
pub struct Clickable;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                track_mouse_clicks,
                log_clicked,
            ).chain())
        ;
    }
}

fn track_mouse_clicks(
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut clicked_event: EventWriter<Clicked>,
    clickable_entities: Query<(Entity, &Aabb, &Transform), With<Clickable>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        let cursor_position = Vec2::new(cursor_position.x, window.height() - cursor_position.y);

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

fn log_clicked(
    mut clicked_event: EventReader<Clicked>,
    entities: Query<&Name>,
) {
    for e in clicked_event.read() {
        if let Ok(name) = entities.get(e.0) {
            println!("clicked on {}!", name);
        }
    }
}
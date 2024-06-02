use bevy::prelude::*;
use crate::{AppState, constants, OnAppState};
use crate::gameplay::field::Field;
use crate::gameplay::plants::price::spawn;

#[derive(Component)]
pub struct CurrentMoney(pub i32);

pub fn spawn_current_money(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    field: Res<Field>,
) {
    let slot_width = 100.0;
    let slots_width: f32 = slot_width * constants::SEED_SLOT_COUNT as f32;
    let slots_width = slots_width + 50.0;

    let spawn_point = field.seed_slots_position + Vec2::new(slots_width, 0.0);
    let spawn_point = Vec3::new(spawn_point.x, spawn_point.y, constants::z_order::SEED_SLOT);

    commands.spawn(Name::new("money"))
        .insert(Transform::from_translation(spawn_point))
        .insert(OnAppState(AppState::Gameplay))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            parent.spawn(Name::new("image"))
                .insert(SpriteBundle {
                    texture: asset_server.load("sprites/RoByn.png"),
                    transform: Transform::from_xyz(-50.0, 0.0, 0.0).with_scale(Vec3::splat(0.5)),
                    ..default()
                });

            spawn::current_money_text(&asset_server, parent, 0, Transform::default());
        })
    ;
}

pub fn update_current_money(
    mut current_money: Query<(&mut Text, &CurrentMoney), Changed<CurrentMoney>>,
) {
    for (mut text, current_money) in current_money.iter_mut() {
        text.sections[0].value = current_money.0.to_string();
    }
}
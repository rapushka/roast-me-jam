use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use crate::constants;
use crate::controls::Input;
use crate::gameplay::field::Field;
use crate::gameplay::seeds::initialization::{get_scale, get_sprite};
use crate::gameplay::seeds::planting::{PlantingState, SelectedSeed};

#[derive(Component, Default)]
pub struct PlantPreview {
    pub can_plant: bool,
}

pub struct PlantPreviewPlugin;

impl Plugin for PlantPreviewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlantingState::Planting), create_plant_preview)

            .add_systems(Update, (
                paint_plant_preview,
                track_position_plant_preview,
            ).run_if(in_state(PlantingState::Planting)))

            .add_systems(OnExit(PlantingState::Planting), destroy_plant_preview)
        ;
    }
}

fn create_plant_preview(
    mut commands: Commands,
    selected_seed: Res<SelectedSeed>,
    asset_server: Res<AssetServer>,
    input: Res<Input>,
) {
    let spawn_point = match input.mouse_world_position {
        Some(cursor_position) => Vec3::new(cursor_position.x, cursor_position.y, 0.0),
        None => Vec3::ZERO,
    };

    let plant_type = selected_seed.0.unwrap();

    commands.spawn(Name::new("plnat preview"))
        .insert(PlantPreview::default())
        .insert(SpriteBundle {
            texture: asset_server.load(get_sprite(plant_type)),
            transform: Transform::from_translation(spawn_point).with_scale(get_scale(plant_type)),
            ..default()
        })
    ;
}

fn destroy_plant_preview(
    mut commands: Commands,
    previews: Query<Entity, With<PlantPreview>>,
) {
    for e in previews.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn paint_plant_preview(
    mut previews: Query<(&mut Sprite, &mut PlantPreview), With<PlantPreview>>,
    field: Res<Field>,
    input: Res<Input>,
) {
    for (mut sprite, mut plant_preview) in previews.iter_mut() {
        let cursor_position = input.mouse_world_position.unwrap_or_default();
        let bounds = field.plants_bounds;

        let valid_position = contains(bounds, cursor_position);

        plant_preview.can_plant = valid_position;
        sprite.color = if valid_position {
            Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.5 }.into()
        } else {
            Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.5 }.into()
        };
    }
}

fn track_position_plant_preview(
    mut previews: Query<&mut Transform, With<PlantPreview>>,
    input: Res<Input>,
) {
    for mut transform in previews.iter_mut() {
        let cursor_position = input.mouse_world_position.unwrap_or_default();
        transform.translation = Vec3::new(cursor_position.x, cursor_position.y, constants::z_order::PLANT_PREVIEW);
    }
}

fn contains(aabb: Aabb2d, point: Vec2) -> bool {
    point.x >= aabb.min.x && point.x <= aabb.max.x && point.y >= aabb.min.y && point.y <= aabb.max.y
}
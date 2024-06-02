use bevy::prelude::*;
use crate::controls::Input;
use crate::gameplay::seeds::initialization::get_sprite;
use crate::gameplay::seeds::planting::{PlantingState, SelectedSeed};

#[derive(Component)]
pub struct PlantPreview;

pub struct PlantPreviewPlugin;

impl Plugin for PlantPreviewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(PlantingState::Planting), create_plant_preview)
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
        .insert(PlantPreview)
        .insert(SpriteBundle {
            texture: asset_server.load(get_sprite(plant_type)),
            transform: Transform::from_translation(spawn_point),
            ..default()
        })
    // .insert(Color::RED)
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
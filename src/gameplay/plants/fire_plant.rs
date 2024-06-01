use bevy::input::ButtonInput;
use bevy::prelude::*;
use crate::controls::Input;
use crate::gameplay::animations::AddAnimationCommand;
use crate::gameplay::plants::{Plant, PlantType, SpawnPlant};
use crate::utils::Vec2Ext;

pub fn test_spawn(
    buttons: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<SpawnPlant>,
) {
    if buttons.just_pressed(KeyCode::KeyF) {
        event.send(SpawnPlant(PlantType::Fire));
    }
}

pub fn spawn(
    mut commands: Commands,
    mut event: EventReader<SpawnPlant>,
    input: Res<Input>,
) {
    for event in event.read() {
        if event.0 != PlantType::Fire {
            continue;
        }

        if let Some(cursor_position) = input.mouse_world_position {
            let entity = commands.spawn(Name::new("fire plant"))
                .insert(Plant(PlantType::Fire))
                .id()
                ;

            commands.add(AddAnimationCommand {
                entity,
                transform: Transform::from_translation(cursor_position.as_vec3()),
                path_to_atlas: "sprites/plants/fire_atlas.png",
                layout: TextureAtlasLayout::from_grid(Vec2::new(150.0, 200.0), 2, 1, None, None),
                fps: 8.0,
                frames_count: 2,
            });
        }
    }
}
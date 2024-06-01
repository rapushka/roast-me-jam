use bevy::asset::AssetServer;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use crate::constants;
use crate::controls::Clickable;
use crate::gameplay::field::Field;
use crate::gameplay::seeds::components::SeedSlot;

pub fn spawn_seeds_slots(
    mut commands: Commands,
    field: Res<Field>,
) {
    let screen_center = field.seed_slots_position;

    commands.add(SpawnSeedsSlotCommand { position: screen_center });
    commands.add(SpawnSeedsSlotCommand { position: screen_center + Vec2::new(100.0, 0.0) });
    commands.add(SpawnSeedsSlotCommand { position: screen_center + Vec2::new(200.0, 0.0) });
}

struct SpawnSeedsSlotCommand {
    pub position: Vec2,
}

impl Command for SpawnSeedsSlotCommand {
    fn apply(self, world: &mut World) {
        let texture = world.resource::<AssetServer>().load("sprites/seed_slot.png");

        let pos = Vec3::new(self.position.x, self.position.y, constants::z_order::SEED_SLOT);

        world.spawn(Name::new("seed slot"))
            .insert(SeedSlot(None))
            .insert(SpriteBundle {
                texture,
                transform: Transform::from_translation(pos),
                ..default()
            })
            .insert(Clickable)
        ;
    }
}
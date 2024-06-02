use bevy::prelude::*;
use crate::{AppState, constants, OnAppState};
use crate::gameplay::field::Field;
use crate::gameplay::plants::price::current_money::CurrentMoney;
use crate::gameplay::score::Score;

#[derive(Component)]
pub struct ScoreView;

pub fn build_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    field: Res<Field>,
) {
    let spawn_point = Vec3::new(125.0, 45.0, constants::z_order::SEED_SLOT);

    commands.spawn(Name::new("score view"))
        .insert(OnAppState(AppState::Gameplay))
        .insert(Transform::from_translation(spawn_point))
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|parent| {
            current_score_text(&asset_server, parent, 0, Transform::default());
        })
    ;
}

pub fn update_score_view(
    score: Res<Score>,
    mut texts: Query<&mut Text, With<ScoreView>>,
) {
    if score.is_changed() {
        for mut text in texts.iter_mut() {
            text.sections[0].value = format!("Score: {}", score.0);
        }
    }
}

fn current_score_text(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    content: i32,
    transform: Transform,
) {
    let font = asset_server.load("fonts/Ldfcomicsans-jj7l.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 64.0,
        color: constants::color::LIGHT_TEXT,
    };
    let text_justification = JustifyText::Left;

    let text = Text::from_section(format!("{}", content.to_string()), text_style.clone())
        .with_justify(text_justification);

    parent.spawn((
        Text2dBundle {
            text,
            transform,
            ..default()
        },
    ))
        .insert(ScoreView)
    ;
}

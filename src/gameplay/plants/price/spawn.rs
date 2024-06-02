use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{default, JustifyText, Res, Text, Text2dBundle, TextStyle, Transform};

use crate::constants;
use crate::gameplay::plants::price::current_money::CurrentMoney;

pub fn price_text(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    content: i32,
    transform: Transform,
) {
    let font = asset_server.load("fonts/Ldfcomicsans-jj7l.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 32.0,
        color: constants::color::GREEN_TEXT,
    };
    let text_justification = JustifyText::Center;

    let text = Text::from_section(format!("{}", content.to_string()), text_style.clone())
        .with_justify(text_justification);

    parent.spawn((
        Text2dBundle {
            text,
            transform,
            ..default()
        },
    ));
}

pub fn current_money_text(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    content: i32,
    transform: Transform,
) {
    let font = asset_server.load("fonts/Ldfcomicsans-jj7l.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 64.0,
        color: constants::color::LIGHT_GREEN_TEXT,
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
        .insert(CurrentMoney(constants::START_MONEY))
    ;
}
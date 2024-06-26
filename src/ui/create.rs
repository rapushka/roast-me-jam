use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::core::Name;
use crate::constants;

pub fn title(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    title_text: String,
) {
    parent.spawn(NodeBundle { style: constants::styles::TITLE, ..default() })
        .with_children(|parent| {
            light_text(asset_server, title_text, parent, 64.0);
        });
}

pub fn button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: String,
    component: C,
)
    where C: Component {
    button_internal(asset_server, parent, string, component, constants::styles::BUTTON);
}

pub fn small_button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: String,
    component: C,
)
    where C: Component {
    button_internal(asset_server, parent, string, component, constants::styles::SMALL_BUTTON);
}

fn button_internal<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    string: String,
    component: C,
    style: Style,
) where C: Component {
    parent.spawn((
        component,
        ButtonBundle {
            style,
            background_color: constants::color::DEFAULT_BUTTON.into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            light_text(asset_server, string, parent, 32.0);
        });
}

pub fn image_button<C>(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    component: C,
    path: &'static str,
    width: f32,
) where C: Component {
    parent.spawn((
        component,
        ButtonBundle {
            style: constants::styles::square_button(width),
            background_color: constants::color::DEFAULT_BUTTON.into(),
            ..default()
        },
    ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(width),
                    ..default()
                },
                image: UiImage::new(asset_server.load(path)),
                ..default()
            });
        });
}

pub fn horizontal_layout(
    parent: &mut ChildBuilder,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                margin: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            ..default()
        }
    ))
        .with_children(spawn_children);
}

pub fn text(
    asset_server: &Res<AssetServer>,
    text: String,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent.spawn((
        Name::new(format!("text: {text}")),
        text_bundle(asset_server, text, font_size),
    ));
}

pub fn light_text(
    asset_server: &Res<AssetServer>,
    text: String,
    parent: &mut ChildBuilder,
    font_size: f32,
) {
    parent.spawn((
        Name::new(format!("text: {text}")),
        light_text_bundle(asset_server, text, font_size),
    ));
}

pub fn text_bundle(
    asset_server: &Res<AssetServer>,
    text: String,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(asset_server, text, font_size, constants::color::DEFAULT_TEXT)
}

pub fn light_text_bundle(
    asset_server: &Res<AssetServer>,
    text: String,
    font_size: f32,
) -> TextBundle {
    colored_text_bundle(asset_server, text, font_size, constants::color::LIGHT_TEXT)
}

fn colored_text_bundle(
    asset_server: &Res<AssetServer>,
    text: String,
    font_size: f32,
    color: Color,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/Ldfcomicsans-jj7l.ttf"),
                        font_size,
                        color,
                    },
                )],
            justify: JustifyText::Center,
            ..default()
        },
        ..default()
    }
}

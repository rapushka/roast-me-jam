pub mod controls {
    use bevy::prelude::KeyCode;

    pub const PAUSE_KEY: KeyCode = KeyCode::Escape;
}

pub mod color {
    use bevy::prelude::*;

    pub const DEFAULT_BUTTON: Color = Color::rgb(0.93, 0.71, 0.32);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.9, 0.81, 0.54);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.89, 0.58, 0.28);

    pub const DEFAULT_TEXT: Color = Color::rgb(0.30, 0.25, 0.22);
    pub const LIGHT_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);
}

pub mod styles {
    use bevy::prelude::*;
    use bevy::ui::AlignSelf::Center;

    pub const LOADING_CURTAIN: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.padding = UiRect::all(Val::Px(10.0));
        style.flex_direction = FlexDirection::ColumnReverse;
        style
    };

    pub const MAIN_MENU: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.row_gap = Val::Px(8.0);
        style
    };
    pub const LEVEL_SELECTION: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.row_gap = Val::Px(8.0);
        style
    };

    pub const GAMEPLAY_HUD: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(100.0);
        style.height = Val::Percent(100.0);
        style.padding = UiRect::all(Val::Px(10.0));
        style
    };

    pub const SPEECH_BUBBLE: Style = {
        let mut style = Style::DEFAULT;
        style.width = Val::Percent(75.0);
        style.height = Val::Percent(20.0);
        style.flex_direction = FlexDirection::Column;
        style.justify_content = JustifyContent::Center;
        style.align_self = AlignSelf::FlexEnd;
        style.align_items = AlignItems::Center;
        style.margin = UiRect {
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Px(20.0),
        };

        style
    };

    pub const BUTTON: Style = {
        let mut style = Style::DEFAULT;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(200.0);
        style.height = Val::Px(80.0);
        style
    };

    pub const SMALL_BUTTON: Style = {
        let mut style = Style::DEFAULT;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(100.0);
        style.height = Val::Px(80.0);
        style
    };

    pub const TITLE: Style = {
        let mut style = Style::DEFAULT;
        style.flex_direction = FlexDirection::Row;
        style.justify_content = JustifyContent::Center;
        style.align_items = AlignItems::Center;
        style.width = Val::Px(300.0);
        style.height = Val::Px(300.0);
        style
    };

    pub fn square_button(size: f32) -> Style {
        Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(size),
            height: Val::Px(size),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        }
    }
}


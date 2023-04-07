use crate::{FontAssets, GameState};
use bevy::prelude::*;

pub struct StartMenuPlugin;
impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_menu.in_schedule(OnEnter(GameState::StartMenu)));
    }
}

fn start_menu(fonts: Res<FontAssets>, mut cmds: Commands) {
    // cmds.spawn((
    //     // Create a TextBundle that has a Text with a single section.
    //     TextBundle::from_section(
    //         // Accepts a `String` or any type that converts into a `String`, such as `&str`
    //         "Ender\nof\nTime",
    //         TextStyle {
    //             font: fonts.slk_norm.clone(),
    //             font_size: 80.0,
    //             color: Color::WHITE,
    //         },
    //     ) // Set the alignment of the Text
    //     .with_text_alignment(TextAlignment::Center)
    //     // Set the style of the TextBundle itself.
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         // position: UiRect {
    //         //     left: Val::Percent(40.0),
    //         //     top: Val::Percent(20.0),
    //         //     ..default()
    //         // },
    //         margin: UiRect::all(Val::Auto),
    //         ..default()
    //     }),
    // ));

    cmds.spawn(NodeBundle {
        style: Style {
            // fill the entire window
            size: Size::all(Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        // background_color: BackgroundColor(Color::BLACK),
        ..Default::default()
    })
    //###
    //### TITLE
    //###
    .with_children(|ui| {
        ui.spawn(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Ender\nof\nTime",
                TextStyle {
                    font: fonts.slk_norm.clone(),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Relative,
                // position: UiRect {
                //     left: Val::Percent(40.0),
                //     top: Val::Percent(20.0),
                //     ..default()
                // },
                // margin: UiRect::all(Val::Auto),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Percent(8.0),
                    bottom: Val::Percent(2.0),
                },
                ..default()
            }),
        );
    })
    //###
    //### NEW GAME BUTTON
    //###
    .with_children(|ui| {
        ui.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        }) // NG BUTTON TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "NEW GAME",
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            );
        });
    })
    //###
    //### LOAD GAME BUTTON
    //###
    .with_children(|ui| {
        ui.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        }) // LG BUTTON TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "LOAD GAME",
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            );
        });
    });
}

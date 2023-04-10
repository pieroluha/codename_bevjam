use crate::{FontAssets, GameState};
use bevy::prelude::*;

pub struct StartMenuPlugin;
impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_menu.in_schedule(OnEnter(GameState::StartMenu)))
            .init_resource::<TimeColors>()
            .add_system(epileptic_time.in_set(OnUpdate(GameState::StartMenu)))
            .add_system(sm_button_logic.in_set(OnUpdate(GameState::StartMenu)))
            .add_system(clean_up_sm.in_schedule(OnExit(GameState::StartMenu)));
    }
}

#[derive(Component)]
struct StartMenu;

#[derive(Component)]
struct UiTextTime {
    duration: f32,
    elapsed_time: f32,
}
impl Default for UiTextTime {
    fn default() -> Self {
        Self {
            duration: 0.5,
            elapsed_time: 0.0,
        }
    }
}
enum ButtType {
    NewGame,
    LoadGame,
}
#[derive(Component)]
struct StartMenuButton(ButtType);
#[derive(Component)]
struct StartMenuText(String);

fn start_menu(fonts: Res<FontAssets>, mut cmds: Commands) {
    let red = Color::hex("#a62219").unwrap();
    let white = Color::hex("#e5e5e6").unwrap();
    // let gray = Color::hex("#6e6d73").unwrap();

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
    .insert(StartMenu)
    //###
    //### THE
    //###
    .with_children(|ui| {
        ui.spawn(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "THE",
                TextStyle {
                    font: fonts.slk_norm.clone(),
                    font_size: 80.0,
                    color: white,
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
                    top: Val::Percent(8.0),
                    ..default()
                },
                ..default()
            }),
        );
    })
    //###
    //### CONCOCTER
    //###
    .with_children(|ui| {
        ui.spawn(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "CONCOCTER",
                TextStyle {
                    font: fonts.slk_norm.clone(),
                    font_size: 80.0,
                    color: red,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Relative,
                margin: UiRect {
                    bottom: Val::Percent(2.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(UiTextTime::default());
    })
    //###
    //### PLAY BUTTON
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
        })
        .insert(StartMenuButton(ButtType::NewGame))
        // NG BUTTON TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "NEW GAME",
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 32.0,
                        color: white,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            )
            .insert(StartMenuText("Play".to_string()));
        });
    });
}

#[derive(Resource)]
struct TimeColors([Color; 5]);
impl Default for TimeColors {
    fn default() -> Self {
        Self([
            Color::hex("#9a0e0e").unwrap(),
            Color::hex("#a62219").unwrap(),
            Color::hex("#b33c24").unwrap(),
            Color::hex("#bd4926").unwrap(),
            Color::hex("#cc6633").unwrap(),
        ])
    }
}

fn epileptic_time(
    time: Res<Time>,
    time_colors: Res<TimeColors>,
    mut que_ui_time: Query<(&mut Text, &mut UiTextTime)>,
) {
    let (mut text, mut text_time) = que_ui_time.single_mut();

    text_time.elapsed_time += time.delta_seconds();

    if text_time.elapsed_time > text_time.duration {
        let idx = fastrand::usize(..time_colors.0.len());
        text.sections[0].style.color = time_colors.0[idx];
        text_time.elapsed_time = 0.0;
    }
}

fn sm_button_logic(
    que_interaction: Query<(&Interaction, &StartMenuButton), Changed<Interaction>>,
    mut que_butt_texts: Query<(&StartMenuText, &mut Text)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, butt_type) in que_interaction.iter() {
        match interaction {
            Interaction::None => {
                for (_, mut text) in que_butt_texts.iter_mut() {
                    text.sections[0].style.color = Color::hex("#e5e5e6").unwrap();
                }
            }
            Interaction::Clicked => next_state.set(GameState::InitNew),
            Interaction::Hovered => {
                for (_, mut text) in que_butt_texts.iter_mut() {
                    text.sections[0].style.color = Color::hex("#a62219").unwrap();
                }
            }
        };
    }
}

// text.sections[0].style.color = Color::hex("#a62219").unwrap();

fn clean_up_sm(que_start_menu: Query<Entity, With<StartMenu>>, mut cmds: Commands) {
    let start_menu = que_start_menu.single();
    cmds.get_entity(start_menu)
        .expect("[ERROR]: No StartMenu entity found")
        .despawn_recursive();
}

// fn lerp_color(start: Color, end: Color, t: f32) -> Color {
//     let a = start.as_rgba_f32();
//     let b = end.as_rgba_f32();

//     let sum = [
//         a[0] + (b[0] - a[0]) * t,
//         a[1] + (b[1] - a[1]) * t,
//         a[2] + (b[2] - a[2]) * t,
//         a[3] + (b[3] - a[3]) * t,
//     ];

//     Color::Rgba {
//         red: sum[0],
//         green: sum[1],
//         blue: sum[2],
//         alpha: sum[3],
//     }
// }

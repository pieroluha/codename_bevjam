use crate::creature::stats::Health;
use crate::player::Player;
use crate::potions::PotionTimer;
use crate::{FontAssets, GameState};

use bevy::prelude::*;

pub struct GameUIPlugin;
impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_game_ui.in_set(OnUpdate(GameState::Playing)))
            .add_system(clean_game_ui.in_schedule(OnExit(GameState::Playing)))
            .add_system(game_over.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(game_over_logic.in_set(OnUpdate(GameState::GameOver)))
            .add_system(clean_up_game_over.in_schedule(OnExit(GameState::GameOver)));
    }
}

#[derive(Component)]
struct GameUI;
#[derive(Component)]
struct HealthText;
#[derive(Component)]
struct TimerText;

fn game_ui(
    que_player: Query<&Health, With<Player>>,
    pot_timer: Res<PotionTimer>,
    fonts: Res<FontAssets>,
    mut cmds: Commands,
) {
    let health = que_player.single();
    let white = Color::hex("#e5e5e6").unwrap();

    cmds.spawn(NodeBundle {
        style: Style {
            // fill the entire window
            size: Size::all(Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            margin: UiRect {
                top: Val::Percent(8.0),
                left: Val::Percent(2.0),
                ..default()
            },
            ..Default::default()
        },
        // background_color: BackgroundColor(Color::BLACK),
        ..Default::default()
    })
    .insert(GameUI)
    .with_children(|ui| {
        ui.spawn(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!("Health: {:.0}", health.0),
                TextStyle {
                    font: fonts.slk_norm.clone(),
                    font_size: 42.0,
                    color: white,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center),
        )
        .insert(HealthText);
    })
    .with_children(|ui| {
        ui.spawn(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                format!("Potion: {:.0}", pot_timer.0.elapsed_secs().floor()),
                TextStyle {
                    font: fonts.slk_norm.clone(),
                    font_size: 42.0,
                    color: white,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center),
        )
        .insert(TimerText);
    });
    // .with_children(|ui| {
    //     ui.spawn(ImageBundle {
    //         image: UiImage::new(cre_ass.heart.clone()),
    //         ..default()
    //     });
    // });
}

fn update_game_ui(
    que_player: Query<&Health, With<Player>>,
    pot_timer: Res<PotionTimer>,
    mut que_health: Query<&mut Text, (With<HealthText>, Without<TimerText>)>,
    mut que_timer: Query<&mut Text, (With<TimerText>, Without<HealthText>)>,
) {
    let p_health = que_player.single();
    let mut health = que_health.single_mut();
    let mut timer = que_timer.single_mut();

    health.sections[0].value = format!("Health: {:.0}", p_health.0);
    timer.sections[0].value = format!("Timer: {:.0}", pot_timer.0.elapsed_secs().floor());
}

fn clean_game_ui(que_game_ui: Query<Entity, With<GameUI>>, mut cmds: Commands) {
    let entity = que_game_ui.single();
    cmds.entity(entity).despawn_recursive();
}

#[derive(Component)]
struct GameOver;

fn game_over(fonts: Res<FontAssets>, mut cmds: Commands) {
    let white = Color::hex("#e5e5e6").unwrap();
    cmds.spawn(ButtonBundle {
        style: Style {
            // fill the entire window
            size: Size::all(Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        background_color: BackgroundColor(Color::hex("#0c0d0c").unwrap()),
        ..default()
    })
    .insert(GameOver)
    .with_children(|ui| {
        ui.spawn(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "GAME OVER",
                TextStyle {
                    font: fonts.slk_bold.clone(),
                    font_size: 128.0,
                    color: white,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            }),
        );
    });
}

fn game_over_logic(
    que_interaction: Query<&Interaction, (Changed<Interaction>, With<GameOver>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in que_interaction.iter() {
        match interaction {
            Interaction::Clicked => next_state.set(GameState::StartMenu),
            _ => (),
        };
    }
}

fn clean_up_game_over(que_game: Query<Entity, With<GameOver>>, mut cmds: Commands) {
    let game_over = que_game.single();
    cmds.entity(game_over).despawn_recursive();
}

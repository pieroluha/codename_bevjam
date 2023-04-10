use crate::creature::stats::*;
use crate::creature::Creature;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::{FontAssets, GameState, WIN_HEIGHT, WIN_WIDTH};
use bevy::prelude::*;

pub struct PotionPlugin;
impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Effects>()
            .init_resource::<PotionTimer>()
            .init_resource::<PotionChoices>()
            .init_resource::<ChosenPotion>()
            .add_system(potion_timer.in_set(OnUpdate(GameState::Playing)))
            .add_system(pick_potion_ui.in_schedule(OnEnter(GameState::PickPotion)))
            .add_system(potion_button_logic.in_set(OnUpdate(GameState::PickPotion)))
            .add_system(clean_up_pot_ui.in_schedule(OnExit(GameState::PickPotion)))
            .add_system(potion_effect_player.in_schedule(OnExit(GameState::PickPotion)))
            .add_system(potion_effect_enemies.in_schedule(OnExit(GameState::PickPotion)))
            .add_system(regen_creatures.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Clone)]
pub struct Potion {
    pub good: (Effect, f32),
    pub bad: (Effect, f32),
}
impl Potion {
    fn new(good: (Effect, f32), bad: (Effect, f32)) -> Self {
        Self { good, bad }
    }

    fn get_desc(&self) -> String {
        format!(
            "    Effect: +{} to {}\n    Side Effect: {} to {}",
            self.good.1,
            self.good.0.get_name(),
            self.bad.1,
            self.bad.0.get_name()
        )
    }
}

#[derive(Resource)]
pub struct PotionTimer(pub Timer);
impl Default for PotionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}

fn potion_timer(
    time: Res<Time>,
    mut potion_timer: ResMut<PotionTimer>,
    mut effects: ResMut<Effects>,
    mut next_state: ResMut<NextState<GameState>>,
    mut potion_choices: ResMut<PotionChoices>,
) {
    potion_timer.0.tick(time.delta());

    if potion_timer.0.just_finished() {
        let (a, b, c, d) = effects.get_effects();
        potion_choices.first = Potion::new(
            (a, fastrand::i32(2..10) as f32),
            (b, -fastrand::i32(0..10) as f32),
        );
        potion_choices.second = Potion::new(
            (c, fastrand::i32(2..10) as f32),
            (d, -fastrand::i32(1..10) as f32),
        );
        next_state.set(GameState::PickPotion);
    }
}

// Bro so sloppy no more tiem I don't even know what I'm doung anymore
#[derive(Resource, Clone)]
struct PotionChoices {
    first: Potion,
    second: Potion,
}
impl Default for PotionChoices {
    fn default() -> Self {
        Self {
            first: Potion::new((Effect::Health, 0.0), (Effect::Health, 0.0)),
            second: Potion::new((Effect::Health, 0.0), (Effect::Health, 0.0)),
        }
    }
}

#[derive(Resource)]
struct ChosenPotion {
    chosen: Potion,
    unchosen: Potion,
}
impl Default for ChosenPotion {
    fn default() -> Self {
        Self {
            chosen: Potion::new((Effect::Health, 0.0), (Effect::Health, 0.0)),
            unchosen: Potion::new((Effect::Health, 0.0), (Effect::Health, 0.0)),
        }
    }
}

#[derive(Component)]
struct PickPotionUi;

#[derive(Component)]
struct PotionButton(u32);

fn pick_potion_ui(fonts: Res<FontAssets>, potion_choices: Res<PotionChoices>, mut cmds: Commands) {
    let white = Color::hex("#e5e5e6").unwrap();
    let mut butt_white = white.clone();
    let mut black = Color::hex("#0c0d0c").unwrap();
    black.set_a(0.8);
    butt_white.set_a(0.0);

    cmds.spawn(NodeBundle {
        style: Style {
            // fill the entire window
            size: Size::all(Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(black),
        ..Default::default()
    })
    .insert(PickPotionUi)
    // First P
    .with_children(|ui| {
        ui.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(WIN_WIDTH), Val::Px(WIN_HEIGHT / 3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect {
                    top: Val::Percent(8.0),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor(butt_white),
            ..default()
        })
        .insert(PotionButton(0))
        // First P TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!(
                        "Potion of {}\n {}",
                        potion_choices.first.good.0.get_name(),
                        potion_choices.first.get_desc(),
                    ),
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 48.0,
                        color: white,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Left),
            );
        });
    })
    // Second P
    .with_children(|ui| {
        ui.spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(WIN_WIDTH), Val::Px(WIN_HEIGHT / 3.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(butt_white),
            ..default()
        })
        .insert(PotionButton(1))
        // Second P TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!(
                        "Potion of {}\n {}",
                        potion_choices.second.good.0.get_name(),
                        potion_choices.second.get_desc(),
                    ),
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 48.0,
                        color: white,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Left),
            );
        });
    });
}

fn potion_button_logic(
    potion_choices: Res<PotionChoices>,
    mut que_interaction: Query<
        (&Interaction, &PotionButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut chosen_potion: ResMut<ChosenPotion>,
) {
    for (interaction, pot_butt, mut bg_color) in que_interaction.iter_mut() {
        match interaction {
            Interaction::None => {
                bg_color.0.set_a(0.0);
            }
            Interaction::Clicked => {
                if pot_butt.0 == 0 {
                    chosen_potion.chosen = potion_choices.first.clone();
                    chosen_potion.unchosen = potion_choices.second.clone();
                } else {
                    chosen_potion.chosen = potion_choices.second.clone();
                    chosen_potion.unchosen = potion_choices.first.clone();
                }
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                bg_color.0.set_a(0.4);
            }
        }
    }
}

fn clean_up_pot_ui(que_pot: Query<Entity, With<PickPotionUi>>, mut cmds: Commands) {
    let pot = que_pot.single();
    cmds.entity(pot).despawn_recursive();
}

fn potion_effect_player(
    chosen_potion: Res<ChosenPotion>,
    mut que_player: Query<
        (&mut Health, &mut Speed, &mut Regen, &mut PhysicalDamage),
        (With<Player>, Without<Enemy>),
    >,
) {
    let (mut health, mut speed, mut regen, mut damage) = que_player.single_mut();
    match chosen_potion.chosen.good.0 {
        Effect::Regen => regen.0 = (regen.0 + chosen_potion.chosen.good.1 as f32 * 0.05).max(0.0),
        Effect::Speed => speed.0 = (speed.0 + chosen_potion.chosen.good.1 as f32).max(0.0),
        Effect::Health => health.0 += chosen_potion.chosen.good.1 as f32,
        Effect::PhysicalDamage => {
            damage.0 = (damage.0 + chosen_potion.chosen.good.1 as f32).max(5.0)
        }
    }
    match chosen_potion.chosen.bad.0 {
        Effect::Regen => regen.0 = (regen.0 + chosen_potion.chosen.bad.1 as f32 * 0.05).max(0.0),
        Effect::Speed => speed.0 = (speed.0 + chosen_potion.chosen.bad.1 as f32).max(0.0),
        Effect::Health => health.0 += chosen_potion.chosen.bad.1 as f32,
        Effect::PhysicalDamage => {
            damage.0 = (damage.0 + chosen_potion.chosen.bad.1 as f32).max(5.0)
        }
    }
}

fn potion_effect_enemies(
    chosen_potion: Res<ChosenPotion>,
    mut que_enemies: Query<
        (&mut Health, &mut Speed, &mut Regen, &mut PhysicalDamage),
        (With<Enemy>, Without<Player>),
    >,
) {
    for (mut health, mut speed, mut regen, mut damage) in que_enemies.iter_mut() {
        match chosen_potion.unchosen.good.0 {
            Effect::Regen => {
                regen.0 = (regen.0 + chosen_potion.unchosen.good.1 as f32 * 0.003).max(0.0)
            }
            Effect::Speed => speed.0 = (speed.0 + chosen_potion.unchosen.good.1 as f32).max(0.0),
            Effect::Health => health.0 += chosen_potion.unchosen.good.1 as f32,
            Effect::PhysicalDamage => {
                damage.0 = (damage.0 + chosen_potion.unchosen.good.1 as f32).max(5.0)
            }
        }
        match chosen_potion.unchosen.bad.0 {
            Effect::Regen => {
                regen.0 = (regen.0 + chosen_potion.unchosen.bad.1 as f32 * 0.003).max(0.0)
            }
            Effect::Speed => speed.0 = (speed.0 + chosen_potion.unchosen.bad.1 as f32).max(0.0),
            Effect::Health => health.0 += chosen_potion.unchosen.bad.1 as f32,
            Effect::PhysicalDamage => {
                damage.0 = (damage.0 + chosen_potion.unchosen.bad.1 as f32).max(5.0)
            }
        }
    }
}

fn regen_creatures(mut que_cre: Query<(&mut Health, &Regen), With<Creature>>) {
    for (mut health, regen) in que_cre.iter_mut() {
        health.0 += regen.0;
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Effect {
    Regen,
    Health,
    Speed,
    PhysicalDamage,
}

impl Effect {
    fn get_name(&self) -> &str {
        match self {
            Self::Regen => "Regen",
            Self::Health => "Health",
            Self::Speed => "Speed",
            Self::PhysicalDamage => "Damage",
        }
    }
}

#[derive(Resource)]
struct Effects([Effect; 4]);
impl Default for Effects {
    fn default() -> Self {
        Self([
            Effect::Regen,
            Effect::Health,
            Effect::Speed,
            Effect::PhysicalDamage,
        ])
    }
}
impl Effects {
    fn get_effects(&mut self) -> (Effect, Effect, Effect, Effect) {
        fastrand::shuffle(&mut self.0);
        (
            self.0[0].clone(),
            self.0[1].clone(),
            self.0[2].clone(),
            self.0[3].clone(),
        )
    }
}

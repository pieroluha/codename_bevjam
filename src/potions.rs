use crate::{FontAssets, GameState};
use bevy::prelude::*;

pub struct PotionPlugin;
impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Effects>()
            .init_resource::<PotionTimer>()
            .init_resource::<PotionChoices>()
            .init_resource::<ChosenPotion>()
            .add_system(potion_timer.in_set(OnUpdate(GameState::Playing)));
    }
}

// TODO: Numerical values
pub struct Potion {
    pub good: Effect,
    pub bad: Effect,
}
impl Potion {
    fn new(good: Effect, bad: Effect) -> Self {
        Self { good, bad }
    }

    fn get_name(&self) -> &str {
        match self.good {
            Effect::Regen => "Regen",
            Effect::Health => "Health",
            Effect::Speed => "Speed",
            Effect::PhysicalDamage => "Damage",
        }
    }
}

#[derive(Resource)]
pub struct PotionTimer(Timer);
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
        potion_choices.first = Potion::new(a, b);
        potion_choices.second = Potion::new(c, d);
        next_state.set(GameState::PickPotion);
    }
}

// Bro so sloppy no more tiem I don't even know what I'm doung anymore
#[derive(Resource)]
struct PotionChoices {
    first: Potion,
    second: Potion,
}
impl Default for PotionChoices {
    fn default() -> Self {
        Self {
            first: Potion::new(Effect::Health, Effect::Health),
            second: Potion::new(Effect::Health, Effect::Health),
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
            chosen: Potion::new(Effect::Health, Effect::Health),
            unchosen: Potion::new(Effect::Health, Effect::Health),
        }
    }
}

#[derive(Component)]
struct PickPotionUi;

fn pick_potion_ui(fonts: Res<FontAssets>, potion_choices: Res<PotionChoices>, mut cmds: Commands) {
    let white = Color::hex("#e5e5e6").unwrap();
    let mut black = Color::hex("#0c0d0c").unwrap();
    black.set_a(0.5);

    cmds.spawn(NodeBundle {
        style: Style {
            // fill the entire window
            size: Size::all(Val::Percent(100.)),
            flex_direction: FlexDirection::Row,
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
        // First P TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!("Potion of {}", potion_choices.first.get_name()),
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 64.0,
                        color: white,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            );
        });
    })
    // Second P
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
        // Second P TEXT
        .with_children(|ui| {
            ui.spawn(
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!("Potion of {}", potion_choices.second.get_name()),
                    TextStyle {
                        font: fonts.slk_norm.clone(),
                        font_size: 64.0,
                        color: white,
                    },
                ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Center),
            );
        });
    });
}

#[derive(Clone)]
pub enum Effect {
    Regen,
    Health,
    Speed,
    PhysicalDamage,
}

impl Effect {
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

use crate::GameState;
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

pub struct Potion {
    pub good: Effect,
    pub bad: Effect,
}
impl Potion {
    fn new(good: Effect, bad: Effect) -> Self {
        Self { good, bad }
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


#[derive(Clone)]
pub enum Effect {
    Regen,
    Health,
    Speed,
    PhysicalDamage,
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

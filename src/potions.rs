#[derive(Clone)]
enum Effect {
    Regen,
    Health,
    Speed,
    Mana,
    PhysicalDamage,
    MagicDamage,
}

struct Effects([Effect; 6]);
impl Default for Effects {
    fn default() -> Self {
        Self([
            Effect::Regen,
            Effect::Health,
            Effect::Speed,
            Effect::Mana,
            Effect::PhysicalDamage,
            Effect::MagicDamage,
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

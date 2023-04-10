use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Health(pub f32);
#[derive(Component)]
pub struct Mana(pub f32);
#[derive(Component)]
pub struct Speed(pub f32);
#[derive(Component)]
pub struct Regen(pub f32);
#[derive(Component)]
pub struct ManaRegen(pub f32);
#[derive(Component)]
pub struct PhysicalDamage(pub f32);
#[derive(Component)]
pub struct MagicalDamage(pub f32);

#[derive(Bundle)]
pub struct CreatureBundle {
    pub marker: Creature,
    pub health: Health,
    pub mana: Mana,
    pub speed: Speed,
    pub regen: Regen,
    pub mana_regen: ManaRegen,
    pub phys_dam: PhysicalDamage,
    pub mag_dam: MagicalDamage,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}
impl CreatureBundle {
    pub fn new(
        h: f32,
        m: f32,
        s: f32,
        r: f32,
        mr: f32,
        pd: f32,
        md: f32,
        sprite_sheet: SpriteSheetBundle,
    ) -> Self {
        Self {
            marker: Creature,
            health: Health(h),
            mana: Mana(m),
            speed: Speed(s),
            regen: Regen(r),
            mana_regen: ManaRegen(mr),
            phys_dam: PhysicalDamage(pd),
            mag_dam: MagicalDamage(md),
            sprite_sheet,
        }
    }
}

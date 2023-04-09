use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::creature_shader;

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
    pub mat_handle: creature_shader::CreatureMaterialHandle,
    #[bundle]
    pub mesh2d_bundle: MaterialMesh2dBundle<creature_shader::CreatureMaterial>,
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
        mat_handle: Handle<creature_shader::CreatureMaterial>,
        mesh2d_bundle: MaterialMesh2dBundle<creature_shader::CreatureMaterial>,
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
            mat_handle: creature_shader::CreatureMaterialHandle(mat_handle),
            mesh2d_bundle,
        }
    }
}

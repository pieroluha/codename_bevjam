use super::creature_anim::CRE_ANIM_TIME;
use super::CRE_SIZE;
use crate::enemy::{Enemy, NewEnemy};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Bundle)]
pub struct CreatureBundle {
    marker: Creature,
    health: Health,
    mana: Mana,
    speed: Speed,
    regen: Regen,
    mana_regen: ManaRegen,
    phys_dam: PhysicalDamage,
    mag_dam: MagicalDamage,
    collider: Collider,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    anim_timer: AnimationTimer,
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
            collider: Collider::cuboid(CRE_SIZE / 2.0, CRE_SIZE / 2.0),
            sprite_sheet,
            anim_timer: AnimationTimer(Timer::from_seconds(CRE_ANIM_TIME, TimerMode::Repeating)),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Creature,
    enemy: Enemy,
    new: NewEnemy,
    health: Health,
    mana: Mana,
    speed: Speed,
    regen: Regen,
    mana_regen: ManaRegen,
    phys_dam: PhysicalDamage,
    mag_dam: MagicalDamage,
    collider: Collider,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    // mass: AdditionalMassProperties,
    friction: Friction,
    velocity: Velocity,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    anim_timer: AnimationTimer,
}
impl EnemyBundle {
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
            enemy: Enemy,
            new: NewEnemy,
            health: Health(h),
            mana: Mana(m),
            speed: Speed(s),
            regen: Regen(r),
            mana_regen: ManaRegen(mr),
            phys_dam: PhysicalDamage(pd),
            mag_dam: MagicalDamage(md),
            collider: Collider::cuboid(CRE_SIZE / 2.0, CRE_SIZE / 2.0),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            // mass: AdditionalMassProperties::Mass(1.0),
            friction: Friction::new(0.2),
            velocity: Velocity::default(),
            sprite_sheet,
            anim_timer: AnimationTimer(Timer::from_seconds(CRE_ANIM_TIME, TimerMode::Repeating)),
        }
    }
}

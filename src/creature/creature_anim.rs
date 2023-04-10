use super::creature_bundle::{AnimationTimer, Creature};
use crate::{player::PlayerWeapon, GameState};
use bevy::prelude::*;

pub struct CreatureAnimPlugin;
impl Plugin for CreatureAnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_creatures.in_set(OnUpdate(GameState::Playing)))
            .add_system(animate_weapon.in_set(OnUpdate(GameState::Playing)));
    }
}

const CRE_IDX_COUNT: usize = 2;
pub const CRE_ANIM_TIME: f32 = 0.4;

fn animate_creatures(
    time: Res<Time>,
    mut que_cre: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<Creature>>,
) {
    for (mut timer, mut sprite) in que_cre.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % CRE_IDX_COUNT;
        }
    }
}

fn animate_weapon(
    time: Res<Time>,
    mut que_weap: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<PlayerWeapon>>,
) {
    for (mut timer, mut sprite) in que_weap.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}

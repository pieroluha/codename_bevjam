use crate::player::PlayerWeapon;
use crate::{creature::stats::*, enemy::Enemy, player::Player};
use crate::{get_direction, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_hit.in_set(OnUpdate(GameState::Playing)))
            .add_system(enemy_bonked.in_set(OnUpdate(GameState::Playing)));
    }
}

fn player_hit(
    mut que_enemy: Query<(&PhysicalDamage, &mut Transform, Entity), (With<Enemy>, Without<Player>)>,
    mut events: EventReader<CollisionEvent>,
    mut que_player: Query<
        (&mut Health, &mut Transform, Entity, &mut TextureAtlasSprite),
        (With<Player>, Without<Enemy>),
    >,
) {
    for event in events.iter() {
        let (mut health, p_trans, player, mut sprite) = que_player.single_mut();
        if let CollisionEvent::Started(e1, e2, _) = event {
            let other = if e1 == &player {
                e2
            } else if e2 == &player {
                e1
            } else {
                return;
            };

            for (pd, mut e_trans, enemy) in que_enemy.iter_mut() {
                if &enemy == other {
                    health.0 -= pd.0;
                    let move_dir = get_direction(
                        e_trans.translation.truncate(),
                        p_trans.translation.truncate(),
                    );
                    sprite.index = 2;
                    e_trans.translation += (-move_dir * 20.0).extend(0.0);
                }
            }
        };
    }
}

fn enemy_bonked(
    que_player: Query<&PhysicalDamage, With<Player>>,
    que_weapon: Query<Entity, (With<PlayerWeapon>, Without<Enemy>)>,
    mut events: EventReader<CollisionEvent>,
    mut que_enemy: Query<
        (&mut Health, Entity, &mut TextureAtlasSprite),
        (With<Enemy>, Without<PlayerWeapon>),
    >,
) {
    for event in events.iter() {
        let player_dam = que_player.single();
        let weap = que_weapon.single();

        if let CollisionEvent::Started(e1, e2, _) = event {
            let other = if e1 == &weap {
                e2
            } else if e2 == &weap {
                e1
            } else {
                return;
            };

            for (mut health, enemy, mut sprite) in que_enemy.iter_mut() {
                if &enemy == other {
                    health.0 -= player_dam.0;
                    sprite.index = 2;
                }
            }
        }
    }
}

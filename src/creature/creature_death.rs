use crate::creature::{stats::Health, Creature};
use crate::enemy::Enemy;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;

pub struct CreatureDeathPlugin;
impl Plugin for CreatureDeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(search_dead_enemy.in_set(OnUpdate(GameState::Playing)))
            .add_system(search_dead_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(clean_creatures.in_schedule(OnEnter(GameState::GameOver)));
    }
}

fn search_dead_enemy(que_enemy: Query<(&Health, Entity), With<Enemy>>, mut cmds: Commands) {
    for (health, enemy) in que_enemy.iter() {
        if health.0 < 0.0 {
            cmds.entity(enemy).despawn_recursive();
        }
    }
}

fn search_dead_player(
    que_player: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let health = que_player.single();
    if health.0 < 0.0 {
        next_state.set(GameState::GameOver);
    }
}

fn clean_creatures(que_cre: Query<Entity, With<Creature>>, mut cmds: Commands) {
    for cre in que_cre.iter() {
        cmds.entity(cre).despawn_recursive()
    }
}

use bevy::prelude::*;
use crate::creature::CreatureBundle;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct Player;

// fn spawn_player(mut cmds: Commands, ) {
// }

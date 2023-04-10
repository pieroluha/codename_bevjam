use bevy::prelude::*;
use crate::creature::stats::*;

pub struct ProjectilePlugin; 
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
    }
}

pub struct SpawnProjectile {
    direction: Vec2,
    speed: f32,
}

fn spawn_projectile(mut event_reader: EventReader<SpawnProjectile>, mut cmds: Commands) {
    for proj in event_reader.iter() {
    }
}

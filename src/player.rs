use crate::creature::{
    CreatureAssets, CreatureBundle, CreatureFaction, CreatureMaterial, CreatureMesh,
};
use crate::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_base.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player_base(
    cre_ass: Res<CreatureAssets>,
    cre_mesh: Res<CreatureMesh>,
    mut cmds: Commands,
    mut mats: ResMut<Assets<CreatureMaterial>>,
) {
    let mat_handle = mats.add(CreatureMaterial::new(
        CreatureFaction::Player,
        cre_ass.player.clone(),
    ));

    let mesh2d_bundle = MaterialMesh2dBundle {
        mesh: cre_mesh.0.clone().into(),
        material: mat_handle.clone(),
        transform: Transform::default().with_scale(Vec3::new(32.0, 32.0, 1.0)),
        ..default()
    };

    cmds.spawn(CreatureBundle::new(
        100.0,
        100.0,
        100.0,
        mat_handle,
        mesh2d_bundle,
    ));
}

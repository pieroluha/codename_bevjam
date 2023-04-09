use crate::creature::{
    stats::*, CreatureAssets, CreatureBundle, CreatureFaction, CreatureMaterial, CreatureMesh,
};
use crate::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Cast,
}

fn spawn_player(
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

    // ### INPUTMAP
    let mut input_map = InputMap::new([
        (KeyCode::W, PlayerAction::MoveUp),
        (KeyCode::S, PlayerAction::MoveDown),
        (KeyCode::A, PlayerAction::MoveLeft),
        (KeyCode::D, PlayerAction::MoveRight),
    ]);
    input_map.insert(MouseButton::Left, PlayerAction::Cast);
    // ###

    cmds.spawn(CreatureBundle::new(
        100.0,
        100.0,
        100.0,
        0.3,
        0.2,
        10.0,
        10.0,
        mat_handle,
        mesh2d_bundle,
    ))
    .insert(Player)
    .insert(InputManagerBundle {
        action_state: ActionState::default(),
        input_map,
    });
}

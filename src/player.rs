use crate::creature::{stats::*, CreatureAssets, CreatureBundle, CRE_SIZE};
use crate::GameState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
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

fn spawn_player(cre_ass: Res<CreatureAssets>, mut cmds: Commands) {
    let sprite_sheet = SpriteSheetBundle {
        texture_atlas: cre_ass.player.clone(),
        sprite: TextureAtlasSprite::new(0),
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
        sprite_sheet,
    ))
    .insert(Player)
    .insert(Collider::cuboid(CRE_SIZE / 2.0, CRE_SIZE / 2.0))
    .insert(InputManagerBundle {
        action_state: ActionState::default(),
        input_map,
    });
}

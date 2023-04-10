use crate::camera::Cursor;
use crate::creature::{CreatureAssets, CreatureBundle, CRE_SIZE};
use crate::player_controller::PlayerAction;
use crate::{get_direction, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(shroom_axe_move.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(cre_ass: Res<CreatureAssets>, mut cmds: Commands) {
    let sprite_sheet = SpriteSheetBundle {
        texture_atlas: cre_ass.player.clone(),
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(0.0, 0.0, 3.0),
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

    let sprite = SpriteBundle {
        texture: cre_ass.shroom_axe.clone(),
        // transform: Transform::from_xyz(CRE_SIZE - 4.0, CRE_SIZE - 4.0, 2.0),
        ..default()
    };

    let weapon = cmds.spawn(ShroomAxeBundle::new(sprite)).id();

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
    .insert(RigidBody::KinematicPositionBased)
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(InputManagerBundle {
        action_state: ActionState::default(),
        input_map,
    });
}

#[derive(Component)]
pub struct PlayerWeapon;

#[derive(Bundle)]
struct ShroomAxeBundle {
    #[bundle]
    sprite: SpriteBundle,
    mark: PlayerWeapon,
    collider: Collider,
    rigid_body: RigidBody,
    collision: ActiveEvents,
}
impl ShroomAxeBundle {
    fn new(sprite: SpriteBundle) -> Self {
        Self {
            sprite,
            mark: PlayerWeapon,
            collider: Collider::cuboid(CRE_SIZE / 2.0, CRE_SIZE / 2.0),
            rigid_body: RigidBody::KinematicPositionBased,
            collision: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

fn shroom_axe_move(cursor: ResMut<Cursor>, mut que_axe: Query<&mut Transform, With<PlayerWeapon>>) {
    let mut axe = que_axe.single_mut();
    axe.translation = cursor.0.extend(axe.translation.z);
} 


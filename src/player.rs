use crate::camera::Cursor;
use crate::creature::{AnimationTimer, CreatureAssets, CreatureBundle, CRE_SIZE};
use crate::player_controller::PlayerAction;
use crate::{GameState, InitNewStatus};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::InitNew)))
            .add_system(shroom_axe_move.in_set(OnUpdate(GameState::Playing)))
            .add_system(clean_up_weap.in_schedule(OnEnter(GameState::GameOver)));
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    cre_ass: Res<CreatureAssets>,
    mut cmds: Commands,
    mut init_new_status: ResMut<InitNewStatus>,
) {
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

    let sprite = SpriteSheetBundle {
        texture_atlas: cre_ass.shroom_axe.clone(),
        sprite: TextureAtlasSprite::new(0),
        // transform: Transform::from_xyz(CRE_SIZE - 4.0, CRE_SIZE - 4.0, 2.0),
        ..default()
    };

    cmds.spawn(ShroomAxeBundle::new(sprite));

    cmds.spawn(CreatureBundle::new(
        100.0,
        100.0,
        100.0,
        0.001,
        0.2,
        20.0,
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

    init_new_status.player_ok = true;
}

#[derive(Component)]
pub struct PlayerWeapon;

#[derive(Bundle)]
struct ShroomAxeBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    mark: PlayerWeapon,
    collider: Collider,
    rigid_body: RigidBody,
    collision: ActiveEvents,
    anim_timer: AnimationTimer,
}
impl ShroomAxeBundle {
    fn new(sprite: SpriteSheetBundle) -> Self {
        Self {
            sprite,
            mark: PlayerWeapon,
            collider: Collider::cuboid(CRE_SIZE / 2.0, CRE_SIZE / 2.0),
            rigid_body: RigidBody::KinematicPositionBased,
            collision: ActiveEvents::COLLISION_EVENTS,
            anim_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}

fn shroom_axe_move(cursor: ResMut<Cursor>, mut que_axe: Query<&mut Transform, With<PlayerWeapon>>) {
    let mut axe = que_axe.single_mut();
    axe.translation = cursor.0.extend(axe.translation.z);
}

fn clean_up_weap(que_weap: Query<Entity, With<PlayerWeapon>>, mut cmds: Commands) {
    let weap = que_weap.single();
    cmds.entity(weap).despawn_recursive();
}

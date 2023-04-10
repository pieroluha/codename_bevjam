use crate::{creature::stats::*, player::*, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::*;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CreatureState>()
            .add_system(player_movement.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Melee,
    Cast,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CreatureState {
    #[default]
    Idle,
    Moving,
    Attacking,
    Casting,
}

// fn player_movement(
//     time: Res<Time>,
//     mut player_state: ResMut<NextState<CreatureState>>,
//     mut que_player: Query<(&mut Velocity, &Speed, &ActionState<PlayerAction>), With<Player>>,
// ) {
//     let (mut trans, speed, action) = que_player.single_mut();

//     let pressed = action.get_pressed();
//     for key in pressed.iter() {
//         let move_dir = match key {
//             PlayerAction::MoveUp => Vec2::Y,
//             PlayerAction::MoveDown => -Vec2::Y,
//             PlayerAction::MoveLeft => -Vec2::X,
//             PlayerAction::MoveRight => Vec2::X,
//             _ => Vec2::ZERO,
//         };

//         let vel = move_dir * speed.0 * time.delta_seconds();
//         trans.linvel += vel;

//         // if player_state.0.unwrap() != CreatureState::Moving {
//         //     player_state.set(CreatureState::Moving)
//         // };
//     }

//     if pressed.is_empty() {
//         trans.linvel = Vec2::ZERO;
//     }
// }

// TODO: Make velocity 0 when doing other states?

// fn player_velocity_state(player_state: Res<State<CreatureState>>) {
//     if player_state.0 != CreatureState::Moving {
//     }
// }

fn player_movement(
    time: Res<Time>,
    mut que_player: Query<(&mut Transform, &Speed, &ActionState<PlayerAction>), With<Player>>,
) {
    let (mut trans, speed, action) = que_player.single_mut();

    let pressed = action.get_pressed();
    for key in pressed.iter() {
        let move_dir = match key {
            PlayerAction::MoveUp => Vec2::Y,
            PlayerAction::MoveDown => -Vec2::Y,
            PlayerAction::MoveLeft => -Vec2::X,
            PlayerAction::MoveRight => Vec2::X,
            _ => Vec2::ZERO,
        };

        let vel = move_dir * speed.0 * time.delta_seconds();
        trans.translation += vel.extend(0.0);
    }
}

// TODO:
// fn cast_magic() {
// }

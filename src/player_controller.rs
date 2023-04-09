use crate::{creature::stats::*, player::*, GameState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerControllerPlugin;
impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement.in_set(OnUpdate(GameState::Playing)));
    }
}

fn player_movement(
    time: Res<Time>,
    mut que_player: Query<(&mut Transform, &Speed, &ActionState<PlayerAction>), With<Player>>,
) {
    let (mut trans, speed, action) = que_player.single_mut();

    let pressed = action.get_pressed();

    let mut move_dir = Vec2::ZERO;
    for key in pressed.iter() {
        if key == &PlayerAction::MoveUp {
            move_dir = Vec2::Y;
        }
        if key == &PlayerAction::MoveDown {
            move_dir = -Vec2::Y;
        }
        if key == &PlayerAction::MoveLeft {
            move_dir = -Vec2::X;
        }
        if key == &PlayerAction::MoveRight {
            move_dir = Vec2::X;
        }

        let vel = move_dir * speed.0 * time.delta_seconds();
        trans.translation += vel.extend(0.0);
    }
}

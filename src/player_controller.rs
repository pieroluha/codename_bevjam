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

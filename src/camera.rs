use crate::{player::Player, GameState, WIN_HEIGHT};
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanCamPlugin::default())
            .add_system(spawn_camera.in_schedule(OnEnter(GameState::Startup)))
            .add_system(camera_follow.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(WIN_HEIGHT / 2.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
    // .insert(PanCam {
    //     grab_buttons: vec![MouseButton::Middle],  // which buttons should drag the camera
    //     enabled: true,        // when false, controls are disabled. See toggle example.
    //     zoom_to_cursor: false, // whether to zoom towards the mouse or the center of the screen
    //     min_scale: 1.0,        // prevent the camera from zooming too far in
    //     max_scale: Some(2.5),  // prevent the camera from zooming too far out
    //     ..default()
    // });
}

const SMOOTH: f32 = 0.08;

fn camera_follow(
    que_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut que_cam: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player = que_player.single();
    let mut cam = que_cam.single_mut();
    cam.translation = cam.translation.lerp(player.translation, SMOOTH);
}

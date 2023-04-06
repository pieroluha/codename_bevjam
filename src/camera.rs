use crate::GameState;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanCamPlugin::default())
            .add_system(spawn_camera.in_schedule(OnEnter(GameState::Startup)));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(512.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera)
        .insert(PanCam {
            grab_buttons: vec![],  // which buttons should drag the camera
            enabled: false,        // when false, controls are disabled. See toggle example.
            zoom_to_cursor: false, // whether to zoom towards the mouse or the center of the screen
            min_scale: 1.0,        // prevent the camera from zooming too far in
            max_scale: Some(40.),  // prevent the camera from zooming too far out
            ..default()
        });
}

// fn camera_settings(mut que_cam: Query<&mut OrthographicProjection, With<MainCamera>>) {
//     let mut cam_proj = que_cam.single_mut();

//     cam_proj.scaling_mode = ScalingMode::FixedVertical(800.0);
// }

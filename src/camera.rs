use crate::{GameState, WIN_HEIGHT};
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
                scaling_mode: ScalingMode::FixedVertical(WIN_HEIGHT / 2.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera)
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Middle],  // which buttons should drag the camera
            enabled: true,        // when false, controls are disabled. See toggle example.
            zoom_to_cursor: false, // whether to zoom towards the mouse or the center of the screen
            min_scale: 1.0,        // prevent the camera from zooming too far in
            max_scale: Some(2.5),  // prevent the camera from zooming too far out
            ..default()
        });
}

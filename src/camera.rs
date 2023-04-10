use crate::{player::Player, GameState, WIN_HEIGHT};
use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(PanCamPlugin::default())
            .init_resource::<Cursor>()
            .add_system(spawn_camera.in_schedule(OnEnter(GameState::Startup)))
            .add_system(mouse_pos.in_set(OnUpdate(GameState::Playing)))
            .add_system(camera_follow.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct Cursor(pub Vec2);

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
    //     grab_buttons: vec![MouseButton::Middle], // which buttons should drag the camera
    //     enabled: true, // when false, controls are disabled. See toggle example.
    //     zoom_to_cursor: false, // whether to zoom towards the mouse or the center of the screen
    //     min_scale: 1.0, // prevent the camera from zooming too far in
    //     max_scale: Some(100.0), // prevent the camera from zooming too far out
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

fn mouse_pos(
    que_win: Query<&Window, With<PrimaryWindow>>,
    que_cam: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    if let Ok(win) = que_win.get_single() {
        let (cam, global_trans) = que_cam.single();
        let cursor_pos = win.cursor_position().unwrap_or_default();
        cursor.0 = cam
            .viewport_to_world_2d(global_trans, cursor_pos)
            .unwrap_or_default();
        // println!("{:#?}", cursor_pos);
    }
}

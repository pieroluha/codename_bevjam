use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod background;
mod camera;
mod collision;
mod creature;
mod enemy;
mod fps;
mod game_ui;
mod player;
mod player_controller;
mod potions;
mod sounds;
mod start_menu;

pub const WIN_WIDTH: f32 = 1280.0;
pub const WIN_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Startup,
    StartMenu,
    InitNew,
    Playing,
    PickPotion,
    Paused,
    GameOver,
}

#[derive(Resource, Default)]
pub struct InitNewStatus {
    background_ok: bool,
    player_ok: bool,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Wambology".into(),
                    resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::hex("#0c0d0c").unwrap()))
    // .add_plugin(WorldInspectorPlugin::default())
    .add_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Startup).continue_to_state(GameState::StartMenu),
    )
    .add_collection_to_loading_state::<_, FontAssets>(GameState::Startup);

    app.init_resource::<InitNewStatus>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(camera::CameraPlugin)
        // .add_plugin(fps::FpsPlugin)
        .add_plugin(start_menu::StartMenuPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugins(creature::CreaturePlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(player_controller::PlayerControllerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(potions::PotionPlugin)
        .add_plugin(game_ui::GameUIPlugin)
        .add_plugin(sounds::SoundsPlugin);

    app.add_system(no_gravity.in_schedule(OnEnter(GameState::StartMenu)))
        .add_system(check_init.in_set(OnUpdate(GameState::InitNew)));

    app.run()
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/slkscr.ttf")]
    pub slk_norm: Handle<Font>,
    #[asset(path = "fonts/slkscrb.ttf")]
    pub slk_bold: Handle<Font>,
}

fn no_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub fn get_direction(from: Vec2, to: Vec2) -> Vec2 {
    (to - from).normalize()
}

fn check_init(init_new_status: Res<InitNewStatus>, mut next_state: ResMut<NextState<GameState>>) {
    if init_new_status.player_ok {
        next_state.set(GameState::Playing);
    }
}

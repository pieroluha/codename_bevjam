use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod background;
mod camera;
mod creature;
mod fps;
mod player;
mod player_controller;
mod potions;
mod start_menu;
mod projectile;

pub const WIN_WIDTH: f32 = 1280.0;
pub const WIN_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Startup,
    StartMenu,
    InitNew,
    InitLoad,
    Playing,
    Paused,
}

// #[derive(Resource, Default)]
// pub struct InitNewStatus {
//     background_ok: bool,
//     player_ok: bool,
//     enemy_ok: bool,
// }

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
    .add_plugin(WorldInspectorPlugin::default())
    .add_state::<GameState>()
    .add_loading_state(
        LoadingState::new(GameState::Startup).continue_to_state(GameState::StartMenu),
    )
    .add_collection_to_loading_state::<_, FontAssets>(GameState::Startup);

    app
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(fps::FpsPlugin)
        .add_plugin(start_menu::StartMenuPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugins(creature::CreaturePlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(player_controller::PlayerControllerPlugin);

    app.run()
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/slkscr.ttf")]
    pub slk_norm: Handle<Font>,
    #[asset(path = "fonts/slkscrb.ttf")]
    pub slk_bold: Handle<Font>,
}

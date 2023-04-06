use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod background;
mod camera;
mod fps;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Startup,
    MainMenu,
    Playing,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Wambology".into(),
                    resolution: (800.0, 800.0).into(),
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
    .add_state::<GameState>();
    // .add_loading_state(LoadingState::new(GameState::Startup).continue_to_state(GameState::MainMenu))

    app.add_plugin(camera::CameraPlugin)
        .add_plugin(fps::FpsPlugin)
        .add_plugin(background::BackgroundPlugin);

    app.run()
}

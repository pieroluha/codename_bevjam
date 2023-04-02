use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct FpsPlugin;
impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system(fps_info);
    }
}

fn fps_info(diag: Res<Diagnostics>, mut egui_ctx: EguiContexts) {
    let fps = diag
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .expect("[ERROR] Failed to get fps diagnostics");

    egui::Window::new("Diagnostics").show(egui_ctx.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.2}", fps.average().unwrap_or(0.0)));
    });
}

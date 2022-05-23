use bevy::prelude::*;
use bevy_egui::{egui::{self, Pos2}, EguiContext};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup);
    }
}

fn setup(mut egui_context: ResMut<EguiContext>) {
    egui::Area::new("Main Menu")
    .show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(
            egui::Layout::bottom_up(egui::Align::Center), |ui| {

                if ui.button("Exit Game").clicked() {
                    
                }
                ui.button("Options");
                ui.button("Start Game");
            });
    });
}


fn exit_button() {}
fn options_button() {}
fn start_game_button() {}
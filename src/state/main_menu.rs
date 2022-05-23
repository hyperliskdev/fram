use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
    }

}

fn setup_menu(mut egui_context: ResMut<EguiContext>) {

    egui_context.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
        
    egui::Area::new("Main Menu").movable(true).show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Label::new("Start Game"));
    });
    
    /*
    There is the -?
        camera.orthographic_projection.[top, bottom, left, right ... ]

        possibly useful??
    */
    
}
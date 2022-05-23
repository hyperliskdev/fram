use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_menu);
        app.add_system(system)
    }

}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {}

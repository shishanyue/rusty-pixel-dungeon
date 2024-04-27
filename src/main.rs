#![feature(associated_type_defaults)]

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    }));
    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new());

}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
}
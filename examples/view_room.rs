use bevy::{prelude::*, window::WindowMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rusty_pixel_dungeon::room::{room_project::{RoomProject, RoomSize}, zero_room::ZeroRoomProject, RoomProjectPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    ..default()
                }),

                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(LdtkPlugin)
    .insert_resource(LevelSelection::index(0))
    .add_plugins(RoomProjectPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, zero_room: Res<ZeroRoomProject>) {
    commands.spawn(Camera2dBundle::default());
    zero_room.spawn(&mut commands, RoomSize::Medium);
}
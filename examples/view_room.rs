use bevy::{prelude::*, window::WindowMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rusty_pixel_dungeon::{
    room::{
        room_project::RoomProject,
        zero_room::{self, ZeroRoomProject, ZeroRoomlMark},
        RoomProjectPlugin, RoomSize,
    },
    RustyPixelDungeonPlugin,
};

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
    .add_plugins(RustyPixelDungeonPlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, change_room_size)
    .run();
}

fn setup(mut commands: Commands, zero_room: Res<ZeroRoomProject>) {
    commands.spawn(Camera2dBundle::default());

    zero_room
        .spawn(
            &mut commands,
            Transform::from_xyz(0., 0., 0.),
            RoomSize::Medium,
        )
        .unwrap();
}

fn change_room_size(
    input: Res<ButtonInput<KeyCode>>,
    mut zero_room_level_set: Query<&mut LevelSet, With<ZeroRoomlMark>>,
) {
    if input.just_pressed(KeyCode::KeyW) {
        *zero_room_level_set.single_mut() =
            LevelSet::from_iids(["59b82910-fec0-11ee-9074-4b4ea633e846"]);
    } else if input.just_pressed(KeyCode::KeyS) {
        *zero_room_level_set.single_mut() = LevelSet::from_iids(zero_room::MEDIUM_SIZE);
    }
}

use bevy::{asset::AssetPath, prelude::*};
use bevy_ecs_ldtk::{assets::LdtkProject, LdtkWorldBundle, LevelSet};

use super::room_project::{RoomProject, RoomSize};

const MEDIUM_SIZE:[&str;1] = ["b8200460-fec0-11ee-8027-39315b1638ca"];


#[derive(Default, Resource)]
pub struct ZeroRoomProject {
    name: String,
    room_hanle: Handle<LdtkProject>,
}

#[derive(Component)]
pub struct ZeroRoomlMark;

impl RoomProject for ZeroRoomProject {
    type RoomProjectMark = ZeroRoomlMark;

    fn build(&self, _: &mut App) {}

    fn load<'a>(
        &mut self,
        asset_server: Res<AssetServer>,
        name: &str,
        path: impl Into<AssetPath<'a>>,
    ) {
        self.name = name.to_string();
        self.room_hanle = asset_server.load(path);
    }

    fn spawn(&self, commands: &mut Commands, room_size: RoomSize) -> Entity {
        match room_size {
            RoomSize::Medium => {
                commands
            .spawn((
                ZeroRoomlMark,
                LdtkWorldBundle {
                    ldtk_handle: self.room_hanle.clone_weak(),
                    level_set:LevelSet::from_iids(MEDIUM_SIZE),
                    ..Default::default()
                },
            ))
            .id()
            },
        }
    }
}

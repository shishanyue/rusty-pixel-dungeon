
use bevy::{asset::AssetPath, prelude::*};
use bevy_ecs_ldtk::{assets::LdtkProject, LdtkWorldBundle, LevelSet};

use super::{error::RoomError, room_project::RoomProject, RoomSize};

pub const MEDIUM_SIZE: [&str; 1] = ["3297cf52-fec0-11ee-906e-e9e1ced38237"];

#[derive(Default, Resource)]
pub struct HomeRoomProject {
    name: String,
    room_hanle: Handle<LdtkProject>,
}

#[derive(Component)]
pub struct HomeRoomlMark;

impl RoomProject for HomeRoomProject {
    type RoomProjectMark = HomeRoomlMark;

    fn build(&self, _: &mut App) {}

    fn load<'a>(
        &mut self,
        asset_server: &Res<AssetServer>,
        name: &str,
        path: impl Into<AssetPath<'a>>,
    ) {
        self.name = name.to_string();
        self.room_hanle = asset_server.load(path);
    }

    fn spawn(
        &self,
        commands: &mut Commands,
        transform: Transform,
        room_size: RoomSize,
    ) -> Result<Entity, RoomError> {
        let level_set = match room_size {
            RoomSize::Medium => LevelSet::from_iids(MEDIUM_SIZE),
            _ => return Err(RoomError::NoSuchSize(room_size)),
        };

        Ok(commands
            .spawn((
                HomeRoomlMark,
                LdtkWorldBundle {
                    ldtk_handle: self.room_hanle.clone(),
                    level_set,
                    transform,
                    ..Default::default()
                },
            ))
            .id())
    }

    fn get(
        &self,
        room_size: RoomSize,
    ) -> Result<(Self::RoomProjectMark, LdtkWorldBundle), RoomError> {
        let level_set = match room_size {
            RoomSize::Medium => LevelSet::from_iids(MEDIUM_SIZE),
            _ => return Err(RoomError::NoSuchSize(room_size)),
        };

        Ok((
            HomeRoomlMark,
            LdtkWorldBundle {
                ldtk_handle: self.room_hanle.clone(),
                level_set,
                ..Default::default()
            },
        ))
    }
}

use bevy::{prelude::*};
use bevy_ecs_ldtk::{assets::LdtkProject, LevelSet};
use serde::{Deserialize, Serialize};

use super::room_project::BasicRoomProject;

#[derive(Debug, Default, Component, Reflect, Serialize, Deserialize,Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub enum HomeRoomPojectSize {
    #[default]
    Medium,
}

#[derive(Component, Default)]
pub struct HomeRoomProject;

impl BasicRoomProject for HomeRoomProject {
    type RoomProjectSize = HomeRoomPojectSize;
    fn get_level_set(&self, _: Self::RoomProjectSize) -> LevelSet {
        LevelSet::from_iids(["3297cf52-fec0-11ee-906e-e9e1ced38237"])
    }
    fn spawn(
        &self,
        entity: &Entity,
        level_set:&mut LevelSet,
        project_size:Self::RoomProjectSize,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        *level_set = self.get_level_set(project_size);
        commands
            .get_entity(*entity)
            .unwrap()
            .insert(asset_server.load::<LdtkProject>("environment/rooms/home_room.ldtk"));
    }
}

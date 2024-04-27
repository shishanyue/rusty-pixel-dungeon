pub mod room_project;
pub mod zero_room;

use std::sync::Arc;

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use crate::bevy_ext::AppExt;

use self::{room_project::RoomProject, zero_room::ZeroRoomProject};

pub struct RoomProjectPlugin;

impl Plugin for RoomProjectPlugin {
    fn build(&self, app: &mut App) {
        app.init_room::<ZeroRoomProject>()
            .add_systems(PreStartup, load_room);
    }
}


fn load_room(asset_server: Res<AssetServer>, mut zero_room_project: ResMut<ZeroRoomProject>) {
    zero_room_project.load(
        asset_server,
        "zero_room",
        "environment/rooms/zero_room.ldtk",
    );
}

/*


pub struct LevelRoomPlugin;

#[derive(Resource, Default)]
pub struct LevelRoomMap {
    level_room: HashMap<String, Handle<LdtkProject>>,
}

impl LevelRoomMap {
    pub fn add_room<'a>(
        &mut self,
        name: &str,
        path: impl Into<AssetPath<'a>>,
        asset_server: &Res<AssetServer>,
    ) {
        self.level_room
            .insert(name.to_string(), asset_server.load(path));
    }

    pub fn get_room(&self,name:&str) -> Option<Handle<LdtkProject>>{
        self.level_room.get(&name.to_string()).cloned()
    }
}

impl Plugin for LevelRoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelRoomMap>()
            .add_systems(Startup, load_room_ldtk);
    }
}

fn load_room_ldtk(asset_server: Res<AssetServer>, mut level_room_map: ResMut<LevelRoomMap>) {
    level_room_map.add_room("zero_room", "environment/levels/rooms/zero_room.ldtk", &asset_server);
}


*/

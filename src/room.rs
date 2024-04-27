pub mod room_project;
pub mod zero_room;
pub mod error;


use bevy::prelude::*;

use crate::bevy_ext::AppExt;

use self::{room_project::RoomProject, zero_room::ZeroRoomProject};


#[derive(Debug)]
pub enum RoomSize {
    Small,
    Medium,
    Big,
}

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

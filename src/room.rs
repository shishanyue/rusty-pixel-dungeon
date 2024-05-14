pub mod error;
pub mod home_room;
pub mod room_project;
pub mod zero_room;

use bevy::prelude::*;

use crate::{bevy_ext::AppExt, system::SystemStatus};

use self::{home_room::HomeRoomProject, room_project::RoomProject, zero_room::ZeroRoomProject};

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
        .init_room::<HomeRoomProject>()
            .add_systems(PreStartup, load_room);
    }
}

fn load_room(
    asset_server: Res<AssetServer>,
    mut zero_room_project: ResMut<ZeroRoomProject>,
    mut home_room_project: ResMut<HomeRoomProject>,
    mut system_status: ResMut<SystemStatus>,
) {
    zero_room_project.load(
        &asset_server,
        "zero_room",
        "environment/rooms/zero_room.ldtk",
    );

    home_room_project.load(
        &asset_server,
        "home_room",
        "environment/rooms/home_room.ldtk",
    );

    system_status.inited_rooms = true;
}

pub mod home_room;
pub mod room_project;

use bevy::prelude::*;
use bevy_ecs_ldtk::{assets::LdtkProject, LevelSet};

use self::{home_room::HomeRoomProject, room_project::BasicRoomProject};

pub struct RoomProjectPlugin;

impl Plugin for RoomProjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, check_room_spawn::<HomeRoomProject>);
    }
}

fn check_room_spawn<T: BasicRoomProject>(
    mut commands: Commands,
    mut room_project_world_query: Query<
        (Entity, &mut LevelSet, &T::RoomProjectSize, &T),
        Without<Handle<LdtkProject>>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut level_set, project_size, project_type) in room_project_world_query.iter_mut() {
        project_type.spawn(
            &entity,
            &mut level_set,
            *project_size,
            &mut commands,
            &asset_server,
        );
    }
}

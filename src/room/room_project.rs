use bevy::{prelude::*};
use bevy_ecs_ldtk::{LevelSet};

pub trait BasicRoomProject
where
    Self: Component + Default,
{
    type RoomProjectSize: Component + Clone + Copy;
    fn get_level_set(&self, room_project_size: Self::RoomProjectSize) -> LevelSet;
    fn spawn(
        &self,
        entity: &Entity,
        level_set: &mut LevelSet,
        project_size: Self::RoomProjectSize,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    );
}

#[derive(Bundle, Default)]
pub struct RoomProjectBundle<T>
where
    T: 'static + BasicRoomProject + Default,
{
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub level_set: LevelSet,
    pub room_project_size: T::RoomProjectSize,
    pub room_project_type: T,
}

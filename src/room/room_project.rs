use bevy::{asset::AssetPath, prelude::*};
use bevy_ecs_ldtk::LdtkWorldBundle;

use super::{error::RoomError, RoomSize};

pub trait RoomProject
where
    Self: Send + Sync + Default + Resource,
{
    type RoomProjectMark: Component;
    //type RoomProjectBundle: Bundle;

    fn build(&self, app: &mut App);

    fn load<'a>(
        &mut self,
        asset_server: Res<AssetServer>,
        name: &str,
        path: impl Into<AssetPath<'a>>,
    );

    fn spawn(
        &self,
        commands: &mut Commands,
        transform: Transform,
        room_size: RoomSize,
    ) -> Result<Entity, RoomError>;

    fn get(
        &self,
        room_size: RoomSize,
    ) -> Result<(Self::RoomProjectMark, LdtkWorldBundle), RoomError>;

    //fn get(&self,transform: Transform) -> RoomProjectBundle;
}

pub type RoomName = String;

use bevy::{asset::AssetPath, prelude::*};

pub trait RoomProject
where
    Self: Send + Sync + Default + Resource,
{
    type RoomProjectMark: Component;
    type RoomProjectBundle: Bundle;

    fn build(&self, app: &mut App);

    fn load<'a>(
        &mut self,
        asset_server: Res<AssetServer>,
        name: &str,
        path: impl Into<AssetPath<'a>>,
    );

    fn spawn(&self, commands: &mut Commands, room_size: RoomSize) -> Entity;

    fn get(&self,transform: Transform) -> RoomProjectBundle;
}

pub type RoomName = String;

pub enum RoomSize {
    Medium
}

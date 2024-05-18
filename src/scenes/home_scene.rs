use super::{Scene, SceneState};
use crate::{
    bevy_ext::AppExt, room::{ home_room::{HomeRoomPojectSize, HomeRoomProject}, room_project::RoomProjectBundle},
};
use bevy::prelude::*;

#[derive(Default)]
pub struct HomeScene;

#[derive(Component)]
struct HomeSceneMark;

impl Scene for HomeScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<HomeSceneMark, _>(SceneState::HomeScene, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(RoomProjectBundle::<HomeRoomProject>{
        room_project_size: HomeRoomPojectSize::Medium,
        ..Default::default()
    });
}

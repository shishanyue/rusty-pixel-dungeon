use super::{Scene, SceneState};
use crate::{
    bevy_ext::AppExt,
    custom::resource::AppResource,
    room::{home_room::HomeRoomProject, room_project::RoomProject, RoomSize},
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

fn setup(mut commands: Commands, home_room_project: Res<HomeRoomProject>) {
    home_room_project.spawn(
        &mut commands,
        Transform::from_xyz(10., 10., 10.),
        RoomSize::Medium,
    ).unwrap();
}

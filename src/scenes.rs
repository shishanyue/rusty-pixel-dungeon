use bevy::prelude::*;

use crate::bevy_ext::AppExt;

use self::{home_scene::HomeScene, start_scene::StartScene, title_scene::TitleScene, welcome_scene::WelcomeScene};

mod title_scene;
mod welcome_scene;
mod home_scene;
mod start_scene;

pub trait Scene: Default {
    fn build(&self, app: &mut App);
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum SceneState {
    #[default]
    None,
    WelcomeScene,
    TitleScene,
    HomeScene,
    StartScene
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>()
            .init_scene::<WelcomeScene>()
            .init_scene::<TitleScene>()
            .init_scene::<HomeScene>()
            .init_scene::<StartScene>()
            .add_systems(Startup, |mut scene_state: ResMut<NextState<SceneState>>| {
                scene_state.set(SceneState::WelcomeScene);
            });
    }
}

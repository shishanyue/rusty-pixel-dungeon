use bevy::prelude::*;

use crate::bevy_ext::AppExt;

use self::{title_scene::TitleScene, welcome_scene::WelcomeScene};

mod title_scene;
mod welcome_scene;

pub trait Scene: Default {
    fn build(&self, app: &mut App);
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum SceneState {
    #[default]
    None,
    WelcomeScene,
    TitleScene,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>()
            .init_scene::<WelcomeScene>()
            .init_scene::<TitleScene>()
            .add_systems(Startup, |mut scene_state: ResMut<NextState<SceneState>>| {
                scene_state.set(SceneState::WelcomeScene);
            });
    }
}

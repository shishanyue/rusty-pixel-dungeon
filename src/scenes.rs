use bevy::prelude::*;

mod welcome_scene;


pub trait Scene:Default{
    fn build(&self, app: &mut App);
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, States)]
pub enum SceneState {
    #[default]
    WelcomeScene,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SceneState>();
    }
}

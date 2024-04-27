use crate::bevy_ext::AppExt;
use autodefault::autodefault;
use bevy::{prelude::*, transform::commands};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct WelcomeScene;

#[derive(Component)]
struct WelcomeSceneMark;

impl Scene for WelcomeScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<WelcomeSceneMark, _>(SceneState::WelcomeScene, setup)
            .add_systems(Update, check_status);
    }
}

#[autodefault]
fn setup(mut commands: Commands) {
    commands.spawn((
        WelcomeSceneMark,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
            },
        },
    ));
}

fn check_status() {}

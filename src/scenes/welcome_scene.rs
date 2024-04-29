use crate::{bevy_ext::AppExt, system::SystemStatus};
use autodefault::autodefault;
use bevy::prelude::*;

use super::{Scene, SceneState};

#[derive(Default)]
pub struct WelcomeScene;

#[derive(Component)]
struct WelcomeSceneMark;

impl Scene for WelcomeScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<WelcomeSceneMark, _>(SceneState::WelcomeScene, setup)
            .add_systems(
                Update,
                check_status.run_if(in_state(SceneState::WelcomeScene)),
            );
    }
}

#[autodefault]
fn setup(mut commands: Commands) {
    commands.spawn((
        WelcomeSceneMark,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_self: JustifySelf::Center,
                position_type: PositionType::Absolute,
            },
            background_color: BackgroundColor(Color::BLACK),
        },
    ));
}

fn check_status(mut scene_state: ResMut<NextState<SceneState>>, system_status: Res<SystemStatus>) {
    if system_status.inited_assets && system_status.inited_rooms {
        scene_state.set(SceneState::TitleScene);
    }
}

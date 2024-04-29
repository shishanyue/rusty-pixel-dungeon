use crate::{
    assets::{AppRes, PixelDungeon, PixelDungeonSigns},
    bevy_ext::AppExt,
};
use autodefault::autodefault;
use bevy::{prelude::*, ui::widget::UiImageSize};

use super::{Scene, SceneState};

#[derive(Default)]
pub struct TitleScene;

#[derive(Component)]
struct TitleSceneMark;

impl Scene for TitleScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<TitleSceneMark, _>(SceneState::TitleScene, setup)
            .add_systems(Update, check_status);
    }
}

fn setup(mut commands: Commands, app_res: Res<AppRes>) {
    commands
        .spawn((
            TitleSceneMark,
            NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Percent(50.),
                    justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                AtlasImageBundle {
                    image: UiImage::new(app_res.banners.texture_handle.clone()),
                    texture_atlas: app_res.banners.pixel_dungeon_atlas.clone(),
                    ..Default::default()
                },
                PixelDungeon,
            )).with_children(|parent|{
                parent.spawn((
                    AtlasImageBundle {
                        style: Style {
                            ..Default::default()
                        },
                        image: UiImage::new(app_res.banners.texture_handle.clone()),
                        texture_atlas: app_res.banners.pixel_dungeon_signs_atlas.clone(),
                        ..Default::default()
                    },
                    PixelDungeonSigns,
                ));
            });
        });
}

fn check_status() {}

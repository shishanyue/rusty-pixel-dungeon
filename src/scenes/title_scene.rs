use crate::{
    assets::{banners::{PixelDungeon, PixelDungeonSigns}, AppRes},
    bevy_ext::AppExt,
    custom_bundle::AtlasButtonBundle,
};
use bevy::prelude::*;

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
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    AtlasImageBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        image: UiImage::new(app_res.banners.texture_handle.clone()),
                        texture_atlas: app_res.banners.pixel_dungeon_atlas.clone(),
                        ..Default::default()
                    },
                    PixelDungeon,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        AtlasImageBundle {
                            style: Style {
                                width: Val::Percent(95.),
                                ..Default::default()
                            },
                            image: UiImage::new(app_res.banners.texture_handle.clone()),
                            texture_atlas: app_res.banners.pixel_dungeon_signs_atlas.clone(),
                            ..Default::default()
                        },
                        PixelDungeonSigns,
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Percent(100.),
                        justify_self: JustifySelf::Center,
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            AtlasButtonBundle {
                                style: Style {
                                    width: Val::Percent(25.),
                                    ..Default::default()
                                },
                                image: UiImage::new(app_res.chrome.texture_handle.clone()),
                                texture_atlas: app_res.chrome.grey_button_tr_atlas.clone(),
                                ..Default::default()
                            },
                            ImageScaleMode::Sliced(app_res.chrome.grey_button_tr_slicer.clone()),
                        ))
                        .with_children(|parent| {});
                });
        });
}

fn check_status() {}

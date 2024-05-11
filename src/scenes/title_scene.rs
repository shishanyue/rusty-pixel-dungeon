use crate::{
    assets::{
        app_res::AppRes,
        banners::{PixelDungeon, PixelDungeonSigns},
    },
    bevy_ext::AppExt,
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
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items:AlignItems::Center,
                    justify_self: JustifySelf::Center,
                    justify_content: JustifyContent::FlexStart,
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
                    ImageBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Percent(35.),
                            margin: UiRect {
                                top: Val::Percent(4.),
                                bottom: Val::Percent(6.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        image: UiImage::new(app_res.banners.pixel_dungeon_handle.clone()),
                        ..Default::default()
                    },
                    PixelDungeon,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Percent(94.),
                                ..Default::default()
                            },
                            image: UiImage::new(app_res.banners.pixel_dungeon_signs_handle.clone()),
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
                        justify_content: JustifyContent::FlexStart,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.)),
                                    ..Default::default()
                                },
                                image: UiImage::new(app_res.chrome.grey_button_tr_handle.clone()),
                                ..Default::default()
                            },
                            ImageScaleMode::Sliced(app_res.chrome.grey_button_tr_slicer.clone()),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Button",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    font:app_res.font.clone()
                                },
                            ));
                        });
                });
        });
}

fn check_status() {}

use crate::{
    bevy_ext::AppExt,
    custom::resource::{
        app_image::banners::{PixelDungeon, PixelDungeonSigns},
        AppResource,
    },
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

fn setup(mut commands: Commands, app_res: Res<AppResource>) {
    let text_white_style = TextStyle {
        font: app_res.app_font.bold.clone(),
        font_size: 40.,
        color: Color::WHITE,
    };
    let text_gold_style = TextStyle {
        font: app_res.app_font.bold.clone(),
        font_size: 40.,
        color: Color::GOLD,
    };

    let button_image = app_res.app_image.chrome.grey_button_tr_handle.clone();
    let slicer = app_res.app_image.chrome.grey_button_tr_slicer.clone();
    let icons = app_res.app_image.icons.clone();

    commands
        .spawn((
            TitleSceneMark,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
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
                        image: UiImage::new(app_res.app_image.banners.pixel_dungeon_handle.clone()),
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
                            image: UiImage::new(
                                app_res.app_image.banners.pixel_dungeon_signs_handle.clone(),
                            ),
                            ..Default::default()
                        },
                        PixelDungeonSigns,
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.),
                        height: Val::Percent(100.),
                        justify_self: JustifySelf::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                justify_self: JustifySelf::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            let button_style = Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Percent(45.),
                                padding: UiRect {
                                    top: Val::Px(5.),
                                    bottom: Val::Px(5.),
                                    left: Val::Px(20.),
                                    right: Val::Px(20.),
                                },
                                ..Default::default()
                            };
                            let icon_style = Style {
                                width: Val::Px(50.),
                                margin: UiRect::right(Val::Px(10.)),
                                ..Default::default()
                            };

                            create_button(
                                parent,
                                button_image.clone(),
                                button_style.clone(),
                                slicer.clone(),
                                icons.enter_handle,
                                icon_style.clone(),
                                "进入地牢",
                                text_white_style.clone(),
                            );
                            create_button(
                                parent,
                                button_image.clone(),
                                button_style.clone(),
                                slicer.clone(),
                                icons.gold_handle,
                                icon_style.clone(),
                                "支持游戏开发",
                                text_gold_style.clone(),
                            );
                        });
                });
        });
}

fn create_button(
    parent: &mut ChildBuilder,
    button_image: Handle<Image>,
    button_style: Style,
    slicer: TextureSlicer,
    icon: Handle<Image>,
    icon_style: Style,
    text: &str,
    text_style: TextStyle,
) {
    parent
        .spawn((
            ButtonBundle {
                style: button_style,
                image: UiImage::new(button_image),
                ..Default::default()
            },
            ImageScaleMode::Sliced(slicer),
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(icon),
                style: icon_style,
                ..Default::default()
            });
            parent.spawn(TextBundle::from_section(text, text_style));
        });
}

fn check_status() {}

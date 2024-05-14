use crate::{
    bevy_ext::AppExt,
    custom::resource::{
        app_image::banners::{PixelDungeon, PixelDungeonSigns},
        AppResource,
    },
    utils::ui::create_button,
};
use bevy::prelude::*;

use super::{Scene, SceneState};

#[derive(Default)]
pub struct TitleScene;

#[derive(Component)]
struct TitleSceneMark;

#[derive(Component)]
pub enum ButtonLabel {
    EnterDungeon,
    Badges,
    Rankings,
    Supporter,
    News,
    Changes,
    Prefs,
    About,
}

impl Scene for TitleScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<TitleSceneMark, _>(SceneState::TitleScene, setup)
            .add_systems(
                Update,
                check_interaction.run_if(in_state(SceneState::TitleScene)),
            );
    }
}

fn setup(mut commands: Commands, app_res: Res<AppResource>) {
    let text_white_style = TextStyle {
        font: app_res.app_font.bold.clone(),
        font_size: 30.,
        color: Color::WHITE,
    };
    let text_gold_style = TextStyle {
        font: app_res.app_font.bold.clone(),
        font_size: 30.,
        color: Color::GOLD,
    };

    let button_image = app_res.app_image.chrome.grey_button_tr_handle.clone();
    let slicer = app_res.app_image.chrome.grey_button_tr_slicer.clone();
    let icons = app_res.app_image.icons.clone();
    let icon_style = Style {
        width: Val::Px(30.),
        margin: UiRect {
            right: Val::Px(10.),
            top: Val::Px(10.),
            bottom: Val::Px(10.),
            ..Default::default()
        },
        ..Default::default()
    };

    let two_button_style = Style {
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

    let three_button_style = Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(30.),
        padding: UiRect {
            top: Val::Px(5.),
            bottom: Val::Px(5.),
            left: Val::Px(20.),
            right: Val::Px(20.),
        },
        ..Default::default()
    };

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
                                top: Val::Percent(1.),
                                bottom: Val::Percent(2.),
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
                            create_button(
                                parent,
                                button_image.clone(),
                                two_button_style.clone(),
                                ButtonLabel::EnterDungeon,
                                slicer.clone(),
                                Some(icons.enter_handle),
                                Some(icon_style.clone()),
                                "进入地牢",
                                text_white_style.clone(),
                            );
                            create_button(
                                parent,
                                button_image.clone(),
                                two_button_style.clone(),
                                ButtonLabel::Supporter,
                                slicer.clone(),
                                Some(icons.gold_handle),
                                Some(icon_style.clone()),
                                "支持游戏开发",
                                text_gold_style.clone(),
                            );
                        });

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
                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Rankings,
                                slicer.clone(),
                                Some(icons.rankings_handle),
                                Some(icon_style.clone()),
                                "排行榜",
                                text_white_style.clone(),
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::News,
                                slicer.clone(),
                                Some(icons.news_handle),
                                Some(icon_style.clone()),
                                "游戏新闻",
                                text_white_style.clone(),
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Prefs,
                                slicer.clone(),
                                Some(icons.prefs_handle),
                                Some(icon_style.clone()),
                                "设置",
                                text_white_style.clone(),
                            );
                        });

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
                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Badges,
                                slicer.clone(),
                                Some(icons.badges_handle),
                                Some(icon_style.clone()),
                                "徽章",
                                text_white_style.clone(),
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::Changes,
                                slicer.clone(),
                                Some(icons.changes_handle),
                                Some(icon_style.clone()),
                                "改动",
                                text_white_style.clone(),
                            );

                            create_button(
                                parent,
                                button_image.clone(),
                                three_button_style.clone(),
                                ButtonLabel::About,
                                slicer.clone(),
                                Some(icons.shpx_handle),
                                Some(icon_style.clone()),
                                "关于",
                                text_white_style.clone(),
                            );
                        });
                });
        });
}

fn check_interaction(
    interaction_query: Query<(&Interaction, &ButtonLabel), Changed<Interaction>>,
    mut scene_state: ResMut<NextState<SceneState>>,
) {
    for (interaction, label) in interaction_query.iter() {
        match label {
            ButtonLabel::EnterDungeon => {
                if *interaction == Interaction::Pressed {
                    scene_state.set(SceneState::StartScene);
                }
            }
            ButtonLabel::Badges => {}
            ButtonLabel::Rankings => {}
            ButtonLabel::Supporter => {}
            ButtonLabel::News => {}
            ButtonLabel::Changes => {}
            ButtonLabel::Prefs => {}
            ButtonLabel::About => {}
        }
    }
}

use crate::{
    assets::{AppRes, PixelDungeon, PixelDungeonSigns},
    bevy_ext::AppExt,
};
use bevy::prelude::*;
use bevy_tween::prelude::*;
use bevy_tween::tween::TargetComponent;

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
    let angle_start = 0.;
    let angle_end = std::f32::consts::PI * 2.;

    let start_x = -300.;
    let end_x = 300.;

    let spacing_y = 100.;
    let offset_y = -(spacing_y * 3.) / 2.;

    // Everything in the same entity
    let y = 0. * spacing_y + offset_y;

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
                        SpanTweenerBundle::new(Duration::from_secs(3))
                            .with_repeat(Repeat::Infinitely),
                        SpanTweenBundle::new(..Duration::from_secs(3)),
                        EaseFunction::SineIn,
                        ComponentTween::new_target(
                            TargetComponent::tweener_entity(),
                            interpolate::Translation {
                                start: Vec3::new(start_x, y, 0.),
                                end: Vec3::new(end_x, y, 0.),
                            },
                        ),
                    ));
                });
        });
}

fn check_status() {}

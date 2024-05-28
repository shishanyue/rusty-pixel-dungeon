use crate::{
    actors::hero::{ActiveHero, HeroType, SelectedHero, SelectedHeroEntity},
    bevy_ext::AppExt,
    custom::resource::AppResource,
    utils::{despawn_screen, ui::create_button},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Panel, PanelState};

#[derive(Default)]
pub struct HeroViewPanel;

#[derive(Component)]
struct HeroViewPanelMark;

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum ButtonLabel {
    Confirm,
}

impl Panel for HeroViewPanel {
    fn build(&self, app: &mut App) {
        app.add_panel_system::<HeroViewPanelMark, _>(PanelState::HeroViewPanel, setup)
            .add_systems(
                Update,
                (
                    check_interaction.run_if(in_state(PanelState::HeroViewPanel)),
                    despawn_screen::<HeroViewPanelMark>
                        .run_if(any_with_component::<ActiveHero>.and_then(run_once())),
                ),
            );
    }
}

fn setup(mut commands: Commands, app_res: Res<AppResource>, selected_hero_type: Res<SelectedHero>) {
    let text_white_style = TextStyle {
        font: app_res.app_font.bold.clone(),
        font_size: 30.,
        color: Color::WHITE,
    };

    let button_image = app_res.app_image.chrome.grey_button_tr_handle.clone();
    let slicer = app_res.app_image.chrome.grey_button_tr_slicer.clone();
    let button_style = Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect::bottom(Val::Percent(2.5)),
        padding: UiRect {
            top: Val::Px(6.),
            bottom: Val::Px(7.),
            left: Val::Px(20.),
            right: Val::Px(20.),
        },
        ..Default::default()
    };

    let splashe = match *selected_hero_type {
        SelectedHero::Unlocked(hero_type) => match hero_type {
            HeroType::Duelist => app_res.app_image.splashes.duelist.clone(),
            HeroType::Huntress => app_res.app_image.splashes.huntress.clone(),
            HeroType::Rogue => app_res.app_image.splashes.rogue.clone(),
            HeroType::Warrior => app_res.app_image.splashes.warrior.clone(),
            HeroType::Mage => app_res.app_image.splashes.mage.clone(),
        },
    };

    let background_image_handle = app_res.app_image.chrome.window_handle.clone();
    let window_slicer = app_res.app_image.chrome.window_slicer.clone();

    commands
        .spawn((
            HeroViewPanelMark,
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
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
                    ImageBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        image: UiImage::new(background_image_handle),
                        ..Default::default()
                    },
                    ImageScaleMode::Sliced(window_slicer),
                ))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                padding: UiRect::all(Val::Px(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    ..Default::default()
                                },
                                image: UiImage::new(splashe),
                                ..Default::default()
                            });

                            create_button(
                                parent,
                                button_image.clone(),
                                button_style.clone(),
                                ButtonLabel::Confirm,
                                slicer.clone(),
                                None,
                                None,
                                "确定",
                                text_white_style.clone(),
                            );
                        });
                });
        });
}

fn check_interaction(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ButtonLabel), Changed<Interaction>>,
    selected_hero_entity: Res<SelectedHeroEntity>,
) {
    for (interaction, label) in interaction_query.iter() {
        match label {
            ButtonLabel::Confirm => {
                if *interaction == Interaction::Pressed {
                    commands
                        .get_entity(selected_hero_entity.entity)
                        .unwrap()
                        .insert(ActiveHero);
                }
            }
        }
    }
}

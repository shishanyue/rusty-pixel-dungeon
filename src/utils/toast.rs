use std::time::Duration;

use bevy::prelude::*;

use crate::{
    bevy_ext::condition::run_once_in_state, custom::resource::AppResource, states::system::SystemState,
};
use bevy_tweening::{lens::UiPositionLens, *};

pub struct ToastPlugin;

#[derive(Debug, Clone)]
pub enum ToastStyle {
    Normal,
}
#[derive(Debug, Event, Clone)]
pub struct ToastMessage {
    pub message: String,
    pub style: ToastStyle,
}

#[derive(Debug, Default, Resource)]
pub struct ToastMessageList {
    toasts: Vec<ToastMessage>,
}

#[derive(Component)]
pub struct ToastNode;
#[derive(Component)]
pub struct ToastImage;
#[derive(Component)]
pub struct ToastText;

impl ToastMessage {
    pub fn new(message: &str, style: ToastStyle) -> Self {
        Self {
            message: message.to_string(),
            style,
        }
    }
}
impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToastMessage>()
            .add_systems(Update, setup.run_if(run_once_in_state(SystemState::Loaded)))
            .add_systems(
                Update,
                (
                    |mut toast_event: EventReader<ToastMessage>,
                     mut toast_message_list: ResMut<ToastMessageList>| {
                        for toast in toast_event.read() {
                            toast_message_list.toasts.push(toast.clone());
                        }
                    },
                    view_toast,
                )
                    .run_if(resource_exists::<ToastMessageList>),
            );
    }
}

fn setup(mut commands: Commands, app_res: Res<AppResource>) {
    let background_image_handle = app_res.app_image.chrome.window_handle.clone();
    let window_slicer = app_res.app_image.chrome.window_slicer.clone();

    // Create a single animation (tween) to move an entity.
    let tween = Tween::new(
        EaseFunction::BackInOut,
        Duration::from_secs(1),
        UiPositionLens {
            start: UiRect {
                top: Val::Px(0.),
                ..Default::default()
            },
            end: UiRect {
                top: Val::Px(-140.),
                ..Default::default()
            },
        },
    );

    commands
        .spawn((
            ToastNode,
            NodeBundle {
                style: Style {
                    max_width: Val::Percent(45.),
                    align_self: AlignSelf::End,
                    justify_self: JustifySelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                z_index: ZIndex::Local(1),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ToastImage,
                    ImageBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(10.)),
                            ..Default::default()
                        },
                        image: UiImage::new(background_image_handle),
                        ..Default::default()
                    },
                    ImageScaleMode::Sliced(window_slicer),
                    Animator::new(tween),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ToastText,
                        TextBundle::from_section(
                            "adasdasdasd",
                            TextStyle {
                                font: app_res.app_font.light.clone(),
                                font_size: 32.,
                                color: Color::WHITE,
                            },
                        ),
                    ));
                });
        });

    commands.init_resource::<ToastMessageList>();
}

fn view_toast(
    mut toast_visibility: Query<&mut Visibility, With<ToastNode>>,
    mut toast_animator: Query<(Entity, &mut Animator<Style>), With<ToastImage>>,
    mut toast_text: Query<&mut Text, With<ToastText>>,
    mut toast_message_list: ResMut<ToastMessageList>,
    mut tween_event: EventReader<TweenCompleted>,
) {
    let tween = Tween::new(
        EaseFunction::ElasticIn,
        Duration::from_secs(1),
        UiPositionLens {
            start: UiRect {
                top: Val::Px(0.),
                ..Default::default()
            },
            end: UiRect {
                top: Val::Px(-140.),
                ..Default::default()
            },
        },
    )
    .then(
        Tween::new(
            EaseFunction::ElasticInOut,
            Duration::from_secs(1),
            UiPositionLens {
                start: UiRect {
                    top: Val::Px(-140.),
                    ..Default::default()
                },
                end: UiRect {
                    top: Val::Px(0.),
                    ..Default::default()
                },
            },
        )
        .with_completed_event(1),
    );

    let mut visibility = toast_visibility.single_mut();
    let (entity, mut animator) = toast_animator.single_mut();
    let mut text = toast_text.single_mut();

    if *visibility == Visibility::Hidden && !toast_message_list.toasts.is_empty() {
        let toast = toast_message_list.toasts.pop().unwrap();
        text.sections[0].value = toast.message;
        animator.set_tweenable(tween);
        *visibility = Visibility::Visible;
    }

    for evnet in tween_event.read() {
        info!("{:?}{:?}",entity,evnet.entity);
        if evnet.entity == entity {
            *visibility = Visibility::Hidden;
        }
    }
}

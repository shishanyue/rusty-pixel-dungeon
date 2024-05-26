use super::{Scene, SceneState};
use crate::{
    bevy_ext::AppExt,
    client::RustyPixelDungeonClient,
    custom::resource::AppResource,
    utils::{
        toast::{ToastMessage, ToastStyle},
        ui::create_button,
    },
};
use bevy::prelude::*;
use bevy_replicon::core::replicon_channels::RepliconChannels;
use rusty_pixel_dungeon_server::server::{
    event::{RoomConfig, RustyPixelDungeonClientNetEvent as NetEvent, Version},
    RustyPixelDungeonServer,
};

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct StartScene;

#[derive(Component)]
struct StartSceneMark;

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum ButtonLabel {
    SinglePlayer,
    MultiPlayer,
}

impl Scene for StartScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<StartSceneMark, _>(SceneState::StartScene, setup)
            .add_systems(
                Update,
                check_interaction.run_if(in_state(SceneState::StartScene)),
            );
    }
}

fn setup(mut commands: Commands, app_res: Res<AppResource>) {
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

    commands
        .spawn((
            StartSceneMark,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            create_button(
                parent,
                button_image.clone(),
                button_style.clone(),
                ButtonLabel::SinglePlayer,
                slicer.clone(),
                None,
                None,
                "单人游戏",
                text_white_style.clone(),
            );

            create_button(
                parent,
                button_image.clone(),
                button_style.clone(),
                ButtonLabel::MultiPlayer,
                slicer.clone(),
                None,
                None,
                "多人游戏",
                text_white_style.clone(),
            );
        });
}

fn check_interaction(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &ButtonLabel), Changed<Interaction>>,
    mut scene_state: ResMut<NextState<SceneState>>,
    server: Res<RustyPixelDungeonServer>,
    client: Res<RustyPixelDungeonClient>,
    channels: Res<RepliconChannels>,
    mut net_evnet: EventWriter<NetEvent>,
    mut toast_evnet: EventWriter<ToastMessage>,
) {
    for (interaction, label) in interaction_query.iter() {
        if interaction == &Interaction::Pressed {
            match label {
                ButtonLabel::SinglePlayer => {
                    toast_evnet.send(ToastMessage::new("正在启动本地服务器", ToastStyle::Normal));
                    server.init_server(&mut commands, &channels).unwrap();
                    client.init_client(&mut commands, &channels).unwrap();

                    net_evnet.send(NetEvent::NewRoom(RoomConfig {
                        is_public: false,
                        max_player: 10,
                        password: "".to_string(),
                        version: Version::V001,
                    }));

                    scene_state.set(SceneState::HomeScene);
                }
                ButtonLabel::MultiPlayer => {}
            }
        }
    }
}

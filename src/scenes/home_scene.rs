use super::{Scene, SceneState};
use crate::{
    actors::hero::{HeroType, SelectedHeroType},
    bevy_ext::AppExt,
    panel::PanelState,
    room::{
        home_room::{HomeRoomPojectSize, HomeRoomProject},
        room_project::{BasicRoomProject, RoomProjectBundle},
    },
};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
#[derive(Default)]
pub struct HomeScene;

#[derive(Component)]
struct HomeSceneMark;

impl Scene for HomeScene {
    fn build(&self, app: &mut App) {
        app.add_scene_system::<HomeSceneMark, _>(SceneState::HomeScene, setup)
            .add_systems(
                Update,
                check_interaction
                    .run_if(in_state(SceneState::HomeScene))
                    .run_if(in_state(PanelState::None)),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        HomeSceneMark,
        RoomProjectBundle::<HomeRoomProject> {
            room_project_size: HomeRoomPojectSize::Medium,
            ..Default::default()
        },
    ));
}
fn check_interaction(
    interaction_query: Query<(&PickingInteraction, &HeroType), Changed<PickingInteraction>>,
    mut selected_hero_type: ResMut<SelectedHeroType>,
    mut panel_state: ResMut<NextState<PanelState>>,
) {
    for (interaction, hero_type) in interaction_query.iter() {
        match interaction {
            PickingInteraction::Pressed => {
                *selected_hero_type = *hero_type;
                panel_state.set(PanelState::HeroViewPanel);
                info!("{:?}  {:?}", interaction, hero_type);
            }
            PickingInteraction::Hovered => {}
            _ => {}
        }
    }
}

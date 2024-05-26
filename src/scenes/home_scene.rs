use super::{Scene, SceneState};
use crate::{
    actors::hero::{ActiveHero, HeroType, SelectedHero, SelectedHeroEntity},
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
                check_interaction.run_if(not(any_with_component::<ActiveHero>).and_then(
                    in_state(SceneState::HomeScene).and_then(in_state(PanelState::None)),
                )),
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
    mut commands: Commands,
    interaction_query: Query<(Entity, &PickingInteraction, &HeroType), Changed<PickingInteraction>>,
    mut selected_hero_type: ResMut<SelectedHero>,
    mut panel_state: ResMut<NextState<PanelState>>,
) {
    for (entity, interaction, hero_type) in interaction_query.iter() {
        match interaction {
            PickingInteraction::Pressed => {
                *selected_hero_type = SelectedHero::Unlocked(*hero_type);
                panel_state.set(PanelState::HeroViewPanel);
                commands.insert_resource(SelectedHeroEntity { entity });
                info!("{:?}  {:?}", interaction, hero_type);
            }
            PickingInteraction::Hovered => {}
            _ => {}
        }
    }
}

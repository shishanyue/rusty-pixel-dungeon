use crate::{actors::hero::{HeroType, SelectedHeroType}, bevy_ext::AppExt, custom::resource::AppResource};

use bevy::prelude::*;

use super::{Panel, PanelState};

#[derive(Default)]
pub struct HeroViewPanel;

#[derive(Component)]
struct HeroViewPanelMark;

impl Panel for HeroViewPanel {
    fn build(&self, app: &mut App) {
        app.add_panel_system::<HeroViewPanelMark, _>(PanelState::HeroViewPanel, setup);
    }
}

fn setup(
    mut commands: Commands,
    app_res: Res<AppResource>,
    selected_hero_type: Res<SelectedHeroType>,
) {
    let splashe = match *selected_hero_type {
        HeroType::Duelist => app_res.app_image.splashes.duelist.clone(),
        HeroType::Huntress => app_res.app_image.splashes.huntress.clone(),
        HeroType::Rogue => app_res.app_image.splashes.rogue.clone(),
        HeroType::Warrior => app_res.app_image.splashes.warrior.clone(),
        HeroType::Mage => app_res.app_image.splashes.mage.clone(),
    };
    commands
        .spawn((
            HeroViewPanelMark,
            NodeBundle {
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    justify_self: JustifySelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(splashe),
                ..Default::default()
            });
        });
}

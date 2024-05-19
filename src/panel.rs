use bevy::prelude::*;

use crate::bevy_ext::AppExt;

use self::hero_view_panel::HeroViewPanel;

pub mod hero_view_panel;

pub trait Panel: Default {
    fn build(&self, app: &mut App);
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum PanelState {
    #[default]
    None,
    HeroViewPanel
}

pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PanelState>()
            .init_panel::<HeroViewPanel>();
    }
}

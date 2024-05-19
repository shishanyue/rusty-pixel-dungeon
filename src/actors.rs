pub mod hero;
use bevy::prelude::*;

use self::hero::SelectedHeroType;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedHeroType>();
    }
}

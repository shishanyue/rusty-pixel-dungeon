pub mod hero;
use bevy::prelude::*;

use self::hero::SelectedHero;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedHero::Unlocked(hero::HeroType::Rogue));
    }
}

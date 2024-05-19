use bevy::prelude::*;
use bevy_replicon::prelude::*;

use super::RustyPixelDungeonClient;

pub struct RustyPixelDungeonClientPlugin;

impl Plugin for RustyPixelDungeonClientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RustyPixelDungeonClient>();
    }
}

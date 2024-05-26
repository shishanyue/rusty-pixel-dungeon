use bevy::prelude::*;

use self::system::SystemPlugin;
pub mod system;
pub mod game;


pub struct StatesPlugin;


impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SystemPlugin));
    }
}
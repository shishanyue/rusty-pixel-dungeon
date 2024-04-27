pub mod error;

pub mod level_project;


mod home_level;
mod zero_level;

use bevy::prelude::*;

use crate::bevy_ext::AppExt;

use self::{ home_level::HomeLevel, level_project::LevelProject, zero_level::ZeroLevel};




#[derive(Resource, Default)]
pub struct LevelServer {

}

pub struct LevelServerPlugin;

impl Plugin for LevelServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_level::<ZeroLevel>()
        .init_level::<HomeLevel>();
    }
}


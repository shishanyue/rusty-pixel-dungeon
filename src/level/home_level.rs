use bevy::prelude::*;

use super::{level_project::NullLevelType, LevelProject};

#[derive(Default,Resource)]
pub struct HomeLevel;


#[derive(Component)]
pub struct HomeLevelMark;


impl LevelProject for HomeLevel {
    type LevelType = NullLevelType;

    type LevelMark = HomeLevelMark;

    fn build(&self,app:&mut App) {
        todo!()
    }
}
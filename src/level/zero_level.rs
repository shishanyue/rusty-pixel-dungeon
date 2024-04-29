use bevy::prelude::*;

use super::{level_project::NullLevelType, LevelProject};

#[derive(Default,Resource)]
pub struct ZeroLevel;


#[derive(Component)]
pub struct ZeroLevelMark;


impl LevelProject for ZeroLevel {
    type LevelType = NullLevelType;

    type LevelMark = ZeroLevelMark;

    fn build(&self,app:&mut App) {
        
    }
}
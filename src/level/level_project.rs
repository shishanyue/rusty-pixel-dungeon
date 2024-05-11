use bevy::prelude::*;

pub trait LevelProject
where
    Self: Send + Sync + Default + Resource,
{
    type LevelType: Component;
    type LevelMark: Component;

    fn build(&self, app: &mut App);
}

#[derive(Component)]
pub struct NullLevelType;

pub type LevelName = String;

use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_mod_picking::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub static HERO_IDENTIFIER: [&str; 5] = ["Duelist", "Huntress", "Rogue", "Warrior", "Mage"];

pub type SelectedHeroType = HeroType;

/// This enum mirrors an equivalent enum in the `LDtk` project called "Equipment".
#[derive(Debug, Default, Component, Reflect, Serialize, Deserialize, Resource,Clone, Copy)]
#[reflect(Component, Serialize, Deserialize)]
pub enum HeroType {
    #[default]
    Duelist,
    Huntress,
    Rogue,
    Warrior,
    Mage,
}

impl HeroType {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        Self::from_str(
            entity_instance
                .get_enum_field("HeroType")
                .expect("expected entity to have non-nullable equipment_drops enums field"),
        )
        .unwrap()
    }
}

#[derive(Debug, Error)]
#[error("the given hero type value doesn't exist")]
pub struct NoSuchHeroType;

impl FromStr for HeroType {
    type Err = NoSuchHeroType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HeroType::*;
        match s {
            "Duelist" => Ok(Duelist),
            "Huntress" => Ok(Huntress),
            "Rogue" => Ok(Rogue),
            "Warrior" => Ok(Warrior),
            "Mage" => Ok(Mage),
            _ => Err(NoSuchHeroType),
        }
    }
}

#[derive(Default, Component)]
pub struct Hero;

#[derive(Default, Bundle, LdtkEntity)]
pub struct HeroBundle {
    player: Hero,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[with(HeroType::from_field)]
    pub hero_type: HeroType,
    pub pickable: PickableBundle,
}
